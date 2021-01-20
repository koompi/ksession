
use iced::{
    button, Background, Color, Vector,
};

pub const BACKGROUND: Color = Color::from_rgb(238.0 / 255.0, 238.0 / 255.0, 238.0 / 255.0);
pub const FOREGROUND: Color = Color::from_rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0);
pub const HOVERED: Color = Color::from_rgb(129.0 / 255.0, 129.0 / 255.0, 129.0 / 255.0);
// pub const ACCENT: Color = Color::from_rgb(15.0 / 255.0, 86.0 / 255.0, 179.0 / 255.0);
// pub const SUCCESS: Color = Color::from_rgb(31.0 / 255.0, 139.0 / 255.0, 36.0 / 255.0);
// pub const WARNING: Color = Color::from_rgb(212.0 / 255.0, 176.0 / 255.0, 17.0 / 255.0);
// pub const ERROR: Color = Color::from_rgb(218.0 / 255.0, 16.0 / 255.0, 11.0 / 255.0);

pub enum CustomButton {
    Default,
    Card,
    SelectedCard,
}

impl button::StyleSheet for CustomButton {
fn active(&self) -> button::Style {
    button::Style {
        text_color: match self {
            CustomButton::Card => HOVERED,
            _ => Color::BLACK,
        },
        background: Some(Background::Color(match self {
            CustomButton::Card | CustomButton::SelectedCard => Color::TRANSPARENT, 
            _ => BACKGROUND,
        })),
        border_radius: match self {
            CustomButton::Card | CustomButton::SelectedCard => 0.0,
            _ => 5.0,
        },
        border_color: Color::TRANSPARENT,
        border_width: 0.0,
        shadow_offset: match self {
            CustomButton::Default => Vector::new(0.5, 1.0),
            _ => Vector::new(0.0, 0.0),
        },
    }
}

fn hovered(&self) -> button::Style {
    let active = self.active();

    match self {
        CustomButton::Card | CustomButton::SelectedCard => button::Style {
            text_color: Color::BLACK,
            ..active
        },
        CustomButton::Default => button::Style {
            background: FOREGROUND.into(),
            ..active
        }
    }
}
}