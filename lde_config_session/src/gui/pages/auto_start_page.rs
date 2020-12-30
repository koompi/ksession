use iced::{
   button, scrollable, Element, Row, Button, Scrollable, Checkbox, Container, Column, Text, Length, Space, HorizontalAlignment,
};
use crate::gui::{CustomButton, CustomContainer, CustomCheckbox};

#[derive(Debug, Clone, Default)]
pub struct AutoStartPage {
   btn_add_state: button::State,
   btn_edit_state: button::State,
   btn_delete_state: button::State,
   scroll_ls_app: scrollable::State,
   ls_apps: Vec<((bool, &'static str, button::State), Vec<(bool, &'static str)>)>,
   selected_app: Option<(usize, usize)>,
}

#[derive(Debug, Clone)]
pub enum AutoStartMsg {
   AutoStartAppToggled(usize, usize, bool),
   AutoStartModuleExpanded(usize, bool),
   BtnAddClicked,
   BtnEditClicked(usize, usize),
   BtnDeleteClicked(usize, usize),
}

impl AutoStartPage {
   pub fn new() -> Self {
      let mut ls_apps = Vec::new();
      ls_apps.push((
         (true, "Global Autostart", button::State::new()),
         vec![
            (true, "AT-SPI D-Bus Bus"),
            (true, "Geoclue Demo agent"),
            (true, "im-launch"),
            (false, "Network"),
            (true, "nm-tray"),
            (true, "Print Queue Applet"),
            (true, "PulseAudio Sound System"),
            (true, "Snap User Application"),
            (true, "Spice vdagent"),
            (true, "upgNotifier"),
            (true, "User Folders Update"),
         ])
      );
      ls_apps.push((
         (true, "LDE Autostart", button::State::new()),
         vec![
            (true, "Qlipper"),
            (true, "XScreensaver"),
         ])
      );
      Self {
         ls_apps,
         ..Default::default()
      }
   }

   pub fn update(&mut self, msg: AutoStartMsg) {
      use AutoStartMsg::*;
      match msg {
         AutoStartAppToggled(outer_idx, inner_idx, is_checked) => {
            if let Some((_, ls_app)) = self.ls_apps.get_mut(outer_idx) {
               if let Some(app) = ls_app.get_mut(inner_idx) {
                  app.0 = is_checked;
                  self.selected_app = Some((outer_idx, inner_idx));
               }
            }
         },
         AutoStartModuleExpanded(idx, is_expanded) => {
            if let Some((header, _)) = self.ls_apps.get_mut(idx) {
               header.0 = is_expanded;
            }
         },
         BtnAddClicked => {}, 
         BtnEditClicked(..) => {},
         BtnDeleteClicked(..) => {},
      }
   }

   pub fn view(&mut self) -> Element<AutoStartMsg> {
      let AutoStartPage {
         btn_add_state,
         btn_edit_state,
         btn_delete_state,
         scroll_ls_app,
         ls_apps,
         selected_app,
      } = self;

      let txt_apps_autostart = Text::new("Autostart Applications").size(14);
      let scrollable_ls_apps = ls_apps.iter_mut().enumerate().fold(Scrollable::new(scroll_ls_app).spacing(12).padding(10).scroller_width(5).scrollbar_width(7), |scroll, (outer_idx, (header, ls_app))| {
         let header_btn = Button::new(&mut header.2, Text::new(format!("{}  {}", if header.0 {"▼"} else {"▲"}, header.1))).on_press(AutoStartMsg::AutoStartModuleExpanded(outer_idx, !header.0)).style(if !header.0 {CustomButton::SelectedText} else {CustomButton::Text});
         let ls_app_sec = if header.0 {
            ls_app.iter_mut().enumerate().fold(Column::new(), |col, (inner_idx, (is_checked, title))| {
               let checkbox = Checkbox::new(*is_checked, *title, move |is| AutoStartMsg::AutoStartAppToggled(outer_idx, inner_idx, is)).spacing(10).style(CustomCheckbox::Default);
               let mut con = Container::new(checkbox).width(Length::Fill).padding(5);
               if let Some(selected_idx) = selected_app {
                  if selected_idx.0 == outer_idx && selected_idx.1 == inner_idx {
                     con = con.style(CustomContainer::ForegroundGray)
                  } 
               }
               col.push(con)
            })
         } else {
            Column::new()
         };

         scroll.push(
            Column::new().spacing(7)
            .push(header_btn)
            .push(
               Row::new()
               .push(Space::with_width(Length::Units(20)))
               .push(ls_app_sec)
            )
         )
      });
      let ls_apps_sec = Container::new(scrollable_ls_apps).width(Length::Fill).height(Length::Fill).style(CustomContainer::ForegroundWhite);
      let btn_add = Button::new(btn_add_state, Text::new("  Add  ").horizontal_alignment(HorizontalAlignment::Center)).width(Length::Units(75)).on_press(AutoStartMsg::BtnAddClicked).style(CustomButton::Default);
      let mut btn_edit = Button::new(btn_edit_state, Text::new("  Edit  ").horizontal_alignment(HorizontalAlignment::Center)).width(Length::Units(75)).style(CustomButton::Default);
      let mut btn_delete = Button::new(btn_delete_state, Text::new("  Delete  ").horizontal_alignment(HorizontalAlignment::Center)).width(Length::Units(75)).style(CustomButton::Default);
      if let Some((outer_idx, inner_idx)) = selected_app {
         btn_edit = btn_edit.on_press(AutoStartMsg::BtnEditClicked(*outer_idx, *inner_idx));
         btn_delete = btn_delete.on_press(AutoStartMsg::BtnDeleteClicked(*outer_idx, *inner_idx));
      }
      let btn_group = Column::new().spacing(10)
         .push(btn_add)
         .push(btn_edit)
         .push(btn_delete);

      Container::new(
         Column::new().spacing(10)
         .push(txt_apps_autostart)
         .push(
            Row::new().spacing(10)
            .push(ls_apps_sec)
            .push(btn_group)
         )
      ).into()
   }
}