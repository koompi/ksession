use iced::{
   pick_list, button, scrollable, Element, Row, PickList, Button, Text, Container, Column, Align,
   Scrollable, Checkbox, Length, Space,
};
use std::fmt::{Display, Formatter, Result};
use crate::gui::{CustomButton, CustomContainer, CustomSelect, CustomCheckbox};
use config::Config;
use super::auto_start::{AutostartItem, AutostartUtils};
use nfd2::Response;

const wm_key: &'static str = "General.window_manager";
const leave_confirm_key: &'static str = "General.leave_confirmation";
const lck_bef_pow_act_key: &'static str = "General.lock_screen_before_power_actions";
const pow_act_aft_lck_delay_key: &'static str = "General.power_actions_after_lock_delay";
const openbox_val: &'static str = "openbox";

#[derive(Debug, Clone, Default)]
pub struct GeneralPage {
   wm_state: pick_list::State<WM>,
   wm_val: WM,
   btn_search_state: button::State,
   k_mods: Vec<(bool, &'static str, ModuleStatus)>,
   k_mods_scroll: scrollable::State,
   selected_mod: Option<usize>,
   btn_start_state: button::State,
   btn_stop_state: button::State,
   ask_confirm_leave: bool,
   lock_screen: bool,
   config: Config
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
   pub fn new(config: Config) -> Self {
      println!("{:#?}", AutostartItem::create_item_map());

      Self {
         k_mods: vec![
            (true, "Compton (X Compositor)", ModuleStatus::Running),
            (false, "Desktop", ModuleStatus::Idle),
            (false, "Global Keyboard Shortcuts", ModuleStatus::Idle),
            (true, "Notification Daemon", ModuleStatus::Running),
            (true, "Panel", ModuleStatus::Running),
            (true, "PolicyKit Handler", ModuleStatus::Running),
            (false, "Power Management", ModuleStatus::Idle),
            (true, "Runner", ModuleStatus::Running),
         ],
         ask_confirm_leave: config.get(leave_confirm_key).unwrap(),
         lock_screen: true,
         config,
         ..Default::default()  
      }
   }

   pub fn update(&mut self, msg: GeneralMsg) {
      use GeneralMsg::*;
      match msg {
         WMChanged(val) => self.wm_val = val,
         SearchClicked => {
            match nfd2::open_file_dialog(None, Some(std::path::Path::new("/usr/bin"))).expect("oh no") {
               Response::Okay(file_path) => self.wm_val = WM::from(file_path.as_os_str().to_str().unwrap()),
               _ => {}
            }
         },
         LdeModToggled(idx, is_checked) => {
            if let Some(k_mod) = self.k_mods.get_mut(idx) {
               self.selected_mod = Some(idx);
               k_mod.0 = is_checked;
               k_mod.2 =if is_checked {
                  ModuleStatus::Running
               } else {
                  ModuleStatus::Idle
               };
            }
         },
         StartModClicked => {
            if let Some(selected_idx) = self.selected_mod {
               self.k_mods.get_mut(selected_idx).unwrap().2 = ModuleStatus::Starting;
            }
         },
         StopModClicked => {
            if let Some(selected_idx) = self.selected_mod {
               self.k_mods.get_mut(selected_idx).unwrap().2 = ModuleStatus::Stopped;
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
         k_mods,
         k_mods_scroll,
         selected_mod,
         btn_start_state,
         btn_stop_state,
         ask_confirm_leave,
         lock_screen,
         ..
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

      let txt_k_mods = Text::new("LDE Modules").size(14);
      let scroll_k_mods = k_mods.iter_mut().enumerate().fold(
         Scrollable::new(k_mods_scroll).spacing(7).padding(10).scroller_width(5).scrollbar_width(7), 
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
      let k_mods_con = Container::new(scroll_k_mods).width(Length::Fill).height(Length::Fill).style(CustomContainer::ForegroundGray);
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
      let k_mods_sec = Container::new(
         Column::new().spacing(10)
         .push(txt_k_mods)
         .push(k_mods_con)
         .push(btn_group)
      ).width(Length::Fill).height(Length::Fill);

      let txt_k_leave_ses = Text::new("LDE Leave Session").size(14);
      let chb_ask_confirm = Checkbox::new(*ask_confirm_leave, "Ask for confirmation to leave session", GeneralMsg::AskConfirmToggled).spacing(10).style(CustomCheckbox::Default);
      let chb_lock_screen = Checkbox::new(*lock_screen, "Lock screen before suspending/hibernating", GeneralMsg::LockScreenToggled).spacing(10).style(CustomCheckbox::Default);
      let k_leave_ses_sec = Container::new(
         Column::new().spacing(10)
         .push(txt_k_leave_ses)
         .push(chb_ask_confirm)
         .push(chb_lock_screen)
      );

      Container::new(
         Column::new().spacing(20)
         .push(wm_sec)
         .push(k_mods_sec)
         .push(k_leave_ses_sec)
      ).into()
   }

   pub fn restore_settings(&mut self) {
      let known_wms: Vec<String> = get_window_manager_list(true).into_iter().map(|wm| wm.to_string()).collect();
      self.wm_val = WM::from(self.config.get_str(wm_key).unwrap_or(openbox_val.to_string()).as_str());
      self.ask_confirm_leave = self.config.get_bool(leave_confirm_key).unwrap_or(false);
      self.lock_screen = self.config.get_bool(lck_bef_pow_act_key).unwrap_or(true);
   }

   pub fn save(&mut self) {
      let mut do_restart = false;
      let prev_items = AutostartItem::create_item_map().iter().filter(|(_, item)| AutostartUtils::is_k_module(item.file()));
      if self.wm_val != WM::from(self.config.get_str(wm_key).unwrap_or(openbox_val.to_string()).as_str()) {
         let new_wm: &str = self.wm_val.into();
         self.config.set(wm_key, new_wm);
         do_restart = true;
      }

      if self.ask_confirm_leave != self.config.get_bool(leave_confirm_key).unwrap_or(false) {
         self.config.set(leave_confirm_key, self.ask_confirm_leave);
         do_restart = true;
      }

      if self.lock_screen != self.config.get_bool(lck_bef_pow_act_key).unwrap_or(true) {
         self.config.set(lck_bef_pow_act_key, self.lock_screen);
         do_restart = true;
      }
   }
}

// dump func
fn get_window_manager_list(only_available: bool) -> Vec<&'static str> {
   vec![
      "kwin_x11",
      "openbox"
   ]
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

impl From<&str> for WM {
   fn from(s: &str) -> Self { 
      use WM::*;

      match s {
         "kwin_x11" => KWin,
         "openbox" => OpenBox,
         _ => LdeWm
      }
   }
}

impl From<WM> for &str {
   fn from(wm: WM) -> Self {
      use WM::*;

      match wm {
         KWin => "kwin_x11",
         OpenBox => "openbox",
         _ => ""
      }
   }
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