use iced::{
    button, Button, Column, Element, Text, Length, Align, Container, Svg, 
};

#[derive(Debug, Clone)]
pub struct ActionButton {
    icon: String,
    name: &'static str,
    btn_state: button::State,
    _type: ActionType,
}

#[derive(Debug, Clone, Copy)]
pub enum ActionMsg {
    ActionClicked
}

impl ActionButton {
    pub fn new(icon: String, name: &'static str, _type: ActionType) -> Self {
        Self {
            icon,
            name,
            btn_state: button::State::new(),
            _type
        }
    }

    pub fn update(&mut self, msg: ActionMsg) {
        match msg {
            ActionMsg::ActionClicked => {}
        }
    }

    pub fn view(&mut self) -> Element<ActionMsg> {
        let icon = Svg::from_path(&self.icon).width(Length::Fill).height(Length::Fill);
        let name = Text::new(self.name);
        let action_con = Container::new(
            Column::new().spacing(5).align_items(Align::Center)
            .push(icon)
            .push(name)
        ).width(Length::Fill).height(Length::Fill).center_x().center_y();
        
        Button::new(&mut self.btn_state, action_con).width(Length::Fill).height(Length::Fill).on_press(ActionMsg::ActionClicked).into()
    }
}

use num_derive::FromPrimitive;   

#[derive(Debug, Clone, FromPrimitive)]
pub enum ActionType {
    Sleep,
    Hibernate,
    Restart,
    Shutdown,
    Logout,
}