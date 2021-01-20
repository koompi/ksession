use crate::gui::{CustomButton, CustomTextInput};
use iced::{
   button, text_input, Align, Button, Column, Container, Element, Length, Row, Space, Text,
   TextInput,
};

#[derive(Debug, Clone, Default)]
pub struct EnvEditPage {
   name_val: String,
   txt_name_state: text_input::State,
   val: String,
   txt_val_state: text_input::State,
   btn_ok_state: button::State,
   btn_cancel_state: button::State,
}

#[derive(Debug, Clone)]
pub enum EnvEditMsg {
   TxtNameChanged(String),
   TxtValChanged(String),
   BtnOkClicked(String, String),
   BtnCancelClicked,
}

impl EnvEditPage {
   pub fn new(name: String, val: String) -> Self {
      Self {
         name_val: name,
         val,
         ..Self::default()
      }
   }

   pub fn update(&mut self, msg: EnvEditMsg) {
      use EnvEditMsg::*;

      match msg {
         TxtNameChanged(val) => self.name_val = val,
         TxtValChanged(val) => self.val = val,
         BtnCancelClicked => {}
         BtnOkClicked(..) => println!("Ok"),
      }
   }

   pub fn view(&mut self) -> Element<EnvEditMsg> {
      let EnvEditPage {
         name_val,
         txt_name_state,
         val,
         txt_val_state,
         btn_ok_state,
         btn_cancel_state,
      } = self;

      let txt_name = TextInput::new(txt_name_state, "", name_val, EnvEditMsg::TxtNameChanged)
         .padding(10)
         .width(Length::Fill)
         .style(CustomTextInput::Default);
      let txt_val = TextInput::new(txt_val_state, "", val, EnvEditMsg::TxtValChanged)
         .padding(10)
         .width(Length::Fill)
         .style(CustomTextInput::Default);
      let mut btn_ok =
         Button::new(btn_ok_state, Text::new("  Okay  ")).style(CustomButton::Default);
      if !name_val.is_empty() && !val.is_empty() {
         btn_ok = btn_ok.on_press(EnvEditMsg::BtnOkClicked(name_val.clone(), val.clone()));
      }
      let btn_cancel = Button::new(btn_cancel_state, Text::new("  Cancel  "))
         .on_press(EnvEditMsg::BtnCancelClicked)
         .style(CustomButton::Default);

      Container::new(
         Column::new()
            .spacing(10)
            .push(
               Row::new()
                  .spacing(10)
                  .align_items(Align::Center)
                  .push(Container::new(Text::new("Name:")).width(Length::Units(65)))
                  .push(txt_name),
            )
            .push(
               Row::new()
                  .spacing(10)
                  .align_items(Align::Center)
                  .push(Container::new(Text::new("Command:")).width(Length::Units(65)))
                  .push(txt_val),
            )
            .push(
               Row::new()
                  .spacing(10)
                  .align_items(Align::Center)
                  .push(Space::with_width(Length::Fill))
                  .push(btn_ok)
                  .push(btn_cancel),
            ),
      )
      .into()
   }
}
