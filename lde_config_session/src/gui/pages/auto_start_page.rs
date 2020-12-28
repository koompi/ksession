use iced::{
   Element, Row
};

#[derive(Debug, Clone, Default)]
pub struct AutoStartPage;

#[derive(Debug, Clone)]
pub enum AutoStartMsg {
   SidebarChanged(usize)
}

impl AutoStartPage {
   pub fn new() -> Self {
      Self
   }

   pub fn update(&mut self, msg: AutoStartMsg) {

   }

   pub fn view(&mut self) -> Element<AutoStartMsg> {
      Row::new().into()
   }
}