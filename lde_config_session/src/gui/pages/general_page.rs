use iced::{
   Element, Row
};

#[derive(Debug, Clone, Default)]
pub struct GeneralPage;

#[derive(Debug, Clone)]
pub enum GeneralMsg {
   SidebarChanged(usize)
}

impl GeneralPage {
   pub fn new() -> Self {
      Self
   }

   pub fn update(&mut self, msg: GeneralMsg) {

   }

   pub fn view(&mut self) -> Element<GeneralMsg> {
      Row::new().into()
   }
}