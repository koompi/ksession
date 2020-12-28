use iced::{
   Element, Row
};

#[derive(Debug, Clone, Default)]
pub struct EnvPage;

#[derive(Debug, Clone)]
pub enum EnvMsg {
   SidebarChanged(usize)
}

impl EnvPage {
   pub fn new() -> Self {
      Self
   }

   pub fn update(&mut self, msg: EnvMsg) {

   }

   pub fn view(&mut self) -> Element<EnvMsg> {
      Row::new().into()
   }
}