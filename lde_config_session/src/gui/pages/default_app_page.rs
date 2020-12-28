use iced::{
   Element, Row
};

#[derive(Debug, Clone, Default)]
pub struct DefaultAppPage;

#[derive(Debug, Clone)]
pub enum DefaultAppMsg {
   SidebarChanged(usize)
}

impl DefaultAppPage {
   pub fn new() -> Self {
      Self
   }

   pub fn update(&mut self, msg: DefaultAppMsg) {

   }

   pub fn view(&mut self) -> Element<DefaultAppMsg> {
      Row::new().into()
   }
}