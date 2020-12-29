use iced::{
   pick_list, button, scrollable, Element, Row, PickList, Button, Text, Container, Column, Align,
   Scrollable, Checkbox, Length, Space,
};
use std::fmt::{Display, Formatter, Result};
use crate::gui::{CustomButton, CustomContainer, CustomSelect, CustomCheckbox};

#[derive(Debug, Clone, Default)]
pub struct GeneralPage {
   wm_state: pick_list::State<WM>,
   wm_val: WM,
   btn_search_state: button::State,
   lde_mods: Vec<(bool, &'static str, ModuleStatus)>,
   lde_mods_scroll: scrollable::State,
   selected_mod: Option<usize>,
   btn_start_state: button::State,
   btn_stop_state: button::State,
   ask_confirm_leave: bool,
   lock_screen: bool,
}

#[derive(Debug, Clone)]
pub enum GeneralMsg {
   WMChanged(WM),
   SearchClicked,
   LdeModToggled(usize, bool),
   StartModClicked,
   StopModClicked,
   AskConfirmToggled(bool),
   LockScreenToggled(bool),
}

impl GeneralPage {
   pub fn new() -> Self {
      Self {
         lde_mods: vec![
            (true, "Compton (X Compositor)", ModuleStatus::Running),
            (false, "Desktop", ModuleStatus::Idle),
            (false, "Global Keyboard Shortcuts", ModuleStatus::Idle),
            (true, "Notification Daemon", ModuleStatus::Running),
            (true, "Panel", ModuleStatus::Running),
            (true, "PolicyKit Handler", ModuleStatus::Running),
            (false, "Power Management", ModuleStatus::Idle),
            (true, "Runner", ModuleStatus::Running),
         ],
         ask_confirm_leave: true,
         lock_screen: true, 
         ..Default::default()  
      }
   }

   pub fn update(&mut self, msg: GeneralMsg) {
      use GeneralMsg::*;
      match msg {
         WMChanged(val) => self.wm_val = val,
         SearchClicked => {},
         LdeModToggled(idx, is_checked) => {
            if let Some(lde_mod) = self.lde_mods.get_mut(idx) {
               self.selected_mod = Some(idx);
               lde_mod.0 = is_checked;
               lde_mod.2 =if is_checked {
                  ModuleStatus::Running
               } else {
                  ModuleStatus::Idle
               };
            }
         },
         StartModClicked => {
            if let Some(selected_idx) = self.selected_mod {
               self.lde_mods.get_mut(selected_idx).unwrap().2 = ModuleStatus::Starting;
            }
         },
         StopModClicked => {
            if let Some(selected_idx) = self.selected_mod {
               self.lde_mods.get_mut(selected_idx).unwrap().2 = ModuleStatus::Stopped;
            }
         },
         AskConfirmToggled(is_checked) => self.ask_confirm_leave = is_checked,
         LockScreenToggled(is_checked) => self.lock_screen = is_checked,
      }
   }

   pub fn view(&mut self) -> Element<GeneralMsg> {
      let GeneralPage {
         wm_state,
         wm_val,
         btn_search_state,
         lde_mods,
         lde_mods_scroll,
         selected_mod,
         btn_start_state,
         btn_stop_state,
         ask_confirm_leave,
         lock_screen,
      } = self;

      let txt_wm_title = Text::new("Window Manager").size(14);
      let pl_wm = PickList::new(wm_state, &WM::ALL[..], Some(*wm_val), GeneralMsg::WMChanged).width(Length::Fill).style(CustomSelect::Default);
      let btn_search = Button::new(btn_search_state, Text::new("  Search  ")).on_press(GeneralMsg::SearchClicked).style(CustomButton::Default);
      let wm_sec = Container::new(
         Column::new().spacing(7)
         .push(txt_wm_title)
         .push(
            Row::new().width(Length::Fill).spacing(10).align_items(Align::Center)
            .push(Container::new(pl_wm).width(Length::Fill))
            .push(btn_search)
         )
      );

      let txt_lde_mods = Text::new("LDE Modules").size(14);
      let scroll_lde_mods = lde_mods.iter_mut().enumerate().fold(
         Scrollable::new(lde_mods_scroll).spacing(7).padding(10).scroller_width(5).scrollbar_width(7), 
         |scroll, (idx, (is_checked, title, state))| {
            let checkbox = Checkbox::new(*is_checked, *title, move |is| GeneralMsg::LdeModToggled(idx, is)).spacing(10).style(CustomCheckbox::Default);
            let row = Row::new().padding(4).align_items(Align::Center)
               .push(checkbox)
               .push(Space::with_width(Length::Fill))
               .push(Text::new(format!("{}", state)));
            let con = Container::new(row).width(Length::Fill);

            scroll.push(
               con.style(if let Some(selected_idx) = selected_mod {
                  if *selected_idx == idx {
                     CustomContainer::Background
                  } else {
                     CustomContainer::Default
                  }
               } else {
                  CustomContainer::Default
               }) 
            )
         }
      );
      let lde_mods_con = Container::new(scroll_lde_mods).width(Length::Fill).height(Length::Fill).style(CustomContainer::ForegroundGray);
      let mut btn_start = Button::new(btn_start_state, Text::new("  Start  ")).style(CustomButton::Default);
      let mut btn_stop = Button::new(btn_stop_state, Text::new("  Stop  ")).style(CustomButton::Default);
      if let Some(_) = selected_mod {
         btn_start = btn_start.on_press(GeneralMsg::StartModClicked);
         btn_stop = btn_stop.on_press(GeneralMsg::StopModClicked);
      };
      let btn_group = Row::new().width(Length::Fill).spacing(10).align_items(Align::Center)
         .push(Space::with_width(Length::Fill))
         .push(btn_start)
         .push(btn_stop);
      let lde_mods_sec = Container::new(
         Column::new().spacing(10)
         .push(txt_lde_mods)
         .push(lde_mods_con)
         .push(btn_group)
      ).width(Length::Fill).height(Length::Fill);

      let txt_lde_leave_ses = Text::new("LDE Leave Session").size(14);
      let chb_ask_confirm = Checkbox::new(*ask_confirm_leave, "Ask for confirmation to leave session", GeneralMsg::AskConfirmToggled).spacing(10).style(CustomCheckbox::Default);
      let chb_lock_screen = Checkbox::new(*lock_screen, "Lock screen before suspending/hibernating", GeneralMsg::LockScreenToggled).spacing(10).style(CustomCheckbox::Default);
      let lde_leave_ses_sec = Container::new(
         Column::new().spacing(10)
         .push(txt_lde_leave_ses)
         .push(chb_ask_confirm)
         .push(chb_lock_screen)
      );

      Container::new(
         Column::new().spacing(20)
         .push(wm_sec)
         .push(lde_mods_sec)
         .push(lde_leave_ses_sec)
      ).into()
   }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WM {
   LdeWm,
   KWin,
   OpenBox,
   I3,
   Jwm,
   Xfwm
}

impl Default for WM {
   fn default() -> Self {
      Self::LdeWm
   }
}

impl WM {
   const ALL: [WM; 6] = [
      WM::LdeWm,
      WM::KWin,
      WM::OpenBox,
      WM::I3,
      WM::Jwm,
      WM::Xfwm,
   ];
}

impl Display for WM {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(
         f,
         "{}",
         match self {
            WM::LdeWm => "LdeWM",
            WM::KWin => "Kwin",
            WM::OpenBox => "Openbox",
            WM::I3 => "i3",
            WM::Jwm => "JWM",
            WM::Xfwm => "XFWM",
         }
      )
   }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleStatus {
   Idle,
   Starting,
   Running,
   Stopped,
}

impl Display for ModuleStatus {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(
         f,
         "{}",
         match self {
            ModuleStatus::Idle => "",
            ModuleStatus::Starting => "Starting",
            ModuleStatus::Running => "Running",
            ModuleStatus::Stopped => "Stopped",
         }
      )
   }
}