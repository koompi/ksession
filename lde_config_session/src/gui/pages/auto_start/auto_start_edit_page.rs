use iced::{
   text_input, button, Button, TextInput, Text, Element, Checkbox, Container, Column, Row, 
   Length, Align, Space,
};
use crate::gui::{CustomTextInput, CustomButton, CustomCheckbox};
use nfd2::Response;

#[derive(Debug, Clone, Default)]
pub struct AutoStartEditPage {
   name_val: String,
   txt_name_state: text_input::State,
   cmd_val: String,
   txt_cmd_state: text_input::State,
   btn_search_state: button::State,
   wait_for_system_tray: bool,
   btn_ok_state: button::State,
   btn_cancel_state: button::State,
}

#[derive(Debug, Clone)]
pub enum AutoStartEditPageMsg {
   TxtNameChanged(String),
   TxtCmdChanged(String),
   BtnSearchClicked,
   BtnOkClicked(bool, String, String),
   BtnCancelClicked,
   WaitForSystemTrayToggled(bool),
}

impl AutoStartEditPage {   
   pub fn new(wait_system_tray: bool, name: String, cmd: String) -> Self { 
      Self {
         name_val: name,
         cmd_val: cmd,
         wait_for_system_tray: wait_system_tray,
         ..Self::default()
      }
   }

   pub fn update(&mut self, msg: AutoStartEditPageMsg) {
      use AutoStartEditPageMsg::*;

      match msg { 
         TxtNameChanged(val) => self.name_val = val,
         TxtCmdChanged(val) => self.cmd_val = val,
         BtnSearchClicked => {
            match nfd2::open_file_dialog(None, Some(std::path::Path::new("/usr/bin"))).expect("oh no") {
               Response::Okay(file_path) => self.cmd_val = file_path.into_os_string().into_string().unwrap(),
               _ => {}
            }
         },
         BtnCancelClicked => {},
         WaitForSystemTrayToggled(is_checked) => self.wait_for_system_tray = is_checked,
         BtnOkClicked(..) => println!("Ok")
      }
   }

   pub fn view(&mut self) -> Element<AutoStartEditPageMsg> {
      let AutoStartEditPage {
         name_val,
         txt_name_state,
         cmd_val,
         txt_cmd_state,
         btn_search_state,
         wait_for_system_tray,
         btn_ok_state,
         btn_cancel_state,
      } = self;

      let txt_name = TextInput::new(txt_name_state, "", name_val, AutoStartEditPageMsg::TxtNameChanged).padding(10).width(Length::Fill).style(CustomTextInput::Default);
      let txt_cmd = TextInput::new(txt_cmd_state, "", cmd_val, AutoStartEditPageMsg::TxtCmdChanged).padding(10).width(Length::Fill).style(CustomTextInput::Default);
      let btn_search = Button::new(btn_search_state, Text::new("  Search  ")).on_press(AutoStartEditPageMsg::BtnSearchClicked).style(CustomButton::Default);
      let chb_wait_for = Checkbox::new(*wait_for_system_tray, "Wait for system tray", AutoStartEditPageMsg::WaitForSystemTrayToggled).spacing(10).style(CustomCheckbox::Default);
      let mut btn_ok = Button::new(btn_ok_state, Text::new("  Okay  ")).style(CustomButton::Default);
      if !name_val.is_empty() && !cmd_val.is_empty() {
         btn_ok = btn_ok.on_press(AutoStartEditPageMsg::BtnOkClicked(*wait_for_system_tray, name_val.clone(), cmd_val.clone()));
      }
      let btn_cancel = Button::new(btn_cancel_state, Text::new("  Cancel  ")).on_press(AutoStartEditPageMsg::BtnCancelClicked).style(CustomButton::Default);

      Container::new(
         Column::new().spacing(10)
         .push(
            Row::new().spacing(10).align_items(Align::Center)
            .push(Container::new(Text::new("Name:")).width(Length::Units(65)))
            .push(txt_name)
         )
         .push(
            Row::new().spacing(10).align_items(Align::Center)
            .push(Container::new(Text::new("Command:")).width(Length::Units(65)))
            .push(txt_cmd)
            .push(btn_search)
         )
         .push(chb_wait_for)
         .push(
            Row::new().spacing(10).align_items(Align::Center)
            .push(Space::with_width(Length::Fill))
            .push(btn_ok)
            .push(btn_cancel)
         )
      ).into()
   }
}