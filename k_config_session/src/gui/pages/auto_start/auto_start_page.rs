use iced::{
   button, scrollable, Element, Row, Button, Scrollable, Checkbox, Container, Column, Text, Length, Space, HorizontalAlignment,
};
use crate::gui::{CustomButton, CustomContainer, CustomCheckbox};
use super::auto_start_edit_page::{AutoStartEditPage, AutoStartEditPageMsg};
use super::auto_start_item::AutostartItem;
use super::auto_start_utils::AutostartUtils;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct AutostartPage {
   btn_add_state: button::State,
   btn_edit_state: button::State,
   btn_delete_state: button::State,
   scroll_ls_app: scrollable::State,
   ls_apps: Vec<((bool, &'static str, button::State), Vec<(bool, String, String)>)>,
   selected_app: Option<(usize, usize)>,
   item_map: HashMap<String, AutostartItem>,
   k_items: Vec<String>,
   global_items: Vec<String>,
   auto_start_edit_page: AutoStartEditPage,
   is_editing: bool,
}

#[derive(Debug, Clone)]
pub enum AutostartMsg {
   AutoStartAppToggled(usize, usize, bool),
   AutoStartModuleExpanded(usize, bool),
   BtnAddClicked,
   BtnEditClicked(usize, usize),
   BtnDeleteClicked(usize, usize),
   EditMsg(AutoStartEditPageMsg),
}

impl AutostartPage {
   pub fn new() -> Self {
      let mut ls_apps = Vec::new();
      ls_apps.push((
         (true, "Global Autostart", button::State::new()),
         vec![
            (true, "AT-SPI D-Bus Bus".to_string(), "at-spi-bus-launcher --launch-immediately".to_string()),
            (true, "Geoclue Demo agent".to_string(), "/usr/lib/geoclue-2.0/demos/agent".to_string()),
            (true, "im-launch".to_string(), "im-launch".to_string()),
            (false, "Network".to_string(), "".to_string()),
            (true, "nm-tray".to_string(), "nm-tray".to_string()),
            (true, "Print Queue Applet".to_string(), "".to_string()),
            (true, "PulseAudio Sound System".to_string(), "pulseaudio --start".to_string()),
            (true, "Snap User Application".to_string(), "".to_string()),
            (true, "Spice vdagent".to_string(), "".to_string()),
            (true, "upgNotifier".to_string(), "".to_string()),
            (true, "User Folders Update".to_string(), "".to_string()),
         ])
      );
      ls_apps.push((
         (true, "LDE Autostart", button::State::new()),
         vec![
            (true, "Qlipper".to_string(), "".to_string()),
            (true, "XScreensaver".to_string(), "xscreensaver".to_string()),
         ])
      );
      let item_map = AutostartItem::create_item_map();
      let mut k_items = Vec::new();
      let mut global_items = Vec::new();
      item_map.iter().for_each(|(key, item)| {
         if !AutostartUtils::is_k_module(item.file()) {
            if AutostartUtils::show_only_in_lde(item.file()) {
               k_items.push(key.clone())
            } else {
               global_items.push(key.clone())
            }
         }
      });

      Self {
         ls_apps,
         item_map,
         k_items,
         global_items,
         auto_start_edit_page: AutoStartEditPage::default(),
         ..Default::default()
      }
   }

   pub fn update(&mut self, msg: AutostartMsg) {
      use AutostartMsg::*;
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
         BtnAddClicked => {
            self.is_editing = true;
         }, 
         BtnEditClicked(outer_idx, inner_idx) => {
            if let Some((_, ls_app)) = self.ls_apps.get_mut(outer_idx) {
               if let Some((wait_system_tray, name, cmd)) = ls_app.get(inner_idx) {
                  self.is_editing = true;
                  self.auto_start_edit_page = AutoStartEditPage::new(*wait_system_tray, name.clone(), cmd.clone());
               }
            }
         },
         BtnDeleteClicked(outer_idx, inner_idx) => {
            if let Some((_, ls_app)) = self.ls_apps.get_mut(outer_idx) {
               let _ = ls_app.remove(inner_idx);
            }
            self.selected_app = None;
         },
         EditMsg(edit_msg) => {
            use AutoStartEditPageMsg::*;
            match edit_msg {
               BtnCancelClicked => self.is_editing = false,
               BtnOkClicked(new_wait_system_tray, new_name, new_cmd) => {
                  if let Some((outer_idx, inner_idx)) = self.selected_app {
                     if let Some((_, ls_app)) = self.ls_apps.get_mut(outer_idx) {
                        if let Some((wait_system_tray, name, cmd)) = ls_app.get_mut(inner_idx) {
                           *wait_system_tray = new_wait_system_tray;
                           *name = new_name;
                           *cmd = new_cmd;
                        }
                     }
                  }
                  else {
                     self.ls_apps.get_mut(1).unwrap().1.push((new_wait_system_tray, new_name, new_cmd));
                  }
                  self.auto_start_edit_page = AutoStartEditPage::default();
                  self.is_editing = false;
               },
               _ => self.auto_start_edit_page.update(edit_msg)
            }
         }
      }
   }

   pub fn view(&mut self) -> Element<AutostartMsg> {
      let AutostartPage {
         btn_add_state,
         btn_edit_state,
         btn_delete_state,
         scroll_ls_app,
         ls_apps,
         selected_app,
         item_map,
         auto_start_edit_page,
         is_editing,
         ..
      } = self;

      if *is_editing {
         auto_start_edit_page.view().map(move |msg| AutostartMsg::EditMsg(msg))
      } else {
         let txt_apps_autostart = Text::new("Autostart Applications").size(14);
         let scrollable_ls_apps = ls_apps.iter_mut().enumerate().fold(Scrollable::new(scroll_ls_app).spacing(12).padding(10).scroller_width(5).scrollbar_width(7), |scroll, (outer_idx, (header, ls_app))| {
            let header_btn = Button::new(&mut header.2, Text::new(format!("{}  {}", if header.0 {"▼"} else {"▲"}, header.1))).on_press(AutostartMsg::AutoStartModuleExpanded(outer_idx, !header.0)).style(if !header.0 {CustomButton::SelectedText} else {CustomButton::Text});
            let ls_app_sec = if header.0 {
               ls_app.iter_mut().enumerate().fold(Column::new(), |col, (inner_idx, (is_checked, title, _))| {
                  let checkbox = Checkbox::new(*is_checked, title.as_str(), move |is| AutostartMsg::AutoStartAppToggled(outer_idx, inner_idx, is)).spacing(10).style(CustomCheckbox::Default);
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
         let btn_add = Button::new(btn_add_state, Text::new("  Add  ").horizontal_alignment(HorizontalAlignment::Center)).width(Length::Units(75)).on_press(AutostartMsg::BtnAddClicked).style(CustomButton::Default);
         let mut btn_edit = Button::new(btn_edit_state, Text::new("  Edit  ").horizontal_alignment(HorizontalAlignment::Center)).width(Length::Units(75)).style(CustomButton::Default);
         let mut btn_delete = Button::new(btn_delete_state, Text::new("  Delete  ").horizontal_alignment(HorizontalAlignment::Center)).width(Length::Units(75)).style(CustomButton::Default);
         if let Some((outer_idx, inner_idx)) = selected_app {
            btn_edit = btn_edit.on_press(AutostartMsg::BtnEditClicked(*outer_idx, *inner_idx));
            btn_delete = btn_delete.on_press(AutostartMsg::BtnDeleteClicked(*outer_idx, *inner_idx));
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

   pub fn restore_settings(&mut self) {

   }

   pub fn save(&mut self) {
      let mut do_restart = false;
      let prev_items: HashMap<String, AutostartItem> = AutostartItem::create_item_map().into_iter().filter(|(_, item)| !AutostartUtils::is_k_module(item.file())).collect();
      let current_items: HashMap<String, AutostartItem> = self.items().into_iter().filter(|(_, item)| !AutostartUtils::is_k_module(item.file())).collect();
      if prev_items.capacity() != current_items.capacity() {
         do_restart = true;
      } else {
         for (key, item) in current_items.iter() {
            if prev_items.contains_key(key) {
               if item.file() != prev_items.get(key).unwrap().file() {
                  do_restart = true;
                  break;
               }
            } else {
               do_restart = true;
               break;
            }
         }
      }

      if do_restart {
         // emit needRestart();
      }
   }

   pub fn items(&mut self) -> HashMap<String, AutostartItem> {
      let mut all_items: HashMap<String, AutostartItem> = HashMap::new();
      self.k_items.iter().for_each(|key| {
         all_items.insert(key.clone(), self.item_map.get(key).unwrap().clone());
      });
      self.global_items.iter().for_each(|key| {
         all_items.insert(key.clone(), self.item_map.get(key).unwrap().clone());
      });
      all_items
   }
}