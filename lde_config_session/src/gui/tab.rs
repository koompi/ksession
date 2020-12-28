use iced::{
    button, Button, Column, Element, Text, Length, Align, Container, Svg, 
};
use super::styles::CustomButton;

#[derive(Debug, Clone)]
pub struct Tab {
    icon: String,
    name: &'static str,
    btn_state: button::State
}

#[derive(Debug, Clone, Copy)]
pub enum TabMsg {
    TabClicked
}

impl Tab {
    pub fn new(icon: String, name: &'static str) -> Self {
        Self {
            icon,
            name,
            btn_state: button::State::new(),
        }
    }

    pub fn update(&mut self, msg: TabMsg) {
        match msg {
            TabMsg::TabClicked => {}
        }
    }

    pub fn view(&mut self, is_selected: bool) -> Element<TabMsg> {
        let icon = Svg::from_path(&self.icon).width(Length::Fill).height(Length::Fill);
        let name = Text::new(self.name);
        let action_con = Container::new(
            Column::new().spacing(5).align_items(Align::Center)
            .push(icon)
            .push(name)
        ).width(Length::Fill).height(Length::Fill).center_x().center_y();
        
        Button::new(&mut self.btn_state, action_con).width(Length::Fill).height(Length::Fill).padding(5).on_press(TabMsg::TabClicked).style(if is_selected {CustomButton::SelectedSidebar} else {CustomButton::Sidebar}).into()
    }
}