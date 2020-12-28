
use iced::{
    button, container, Background, Color, Vector,
};

pub const BACKGROUND: Color = Color::from_rgb(238.0 / 255.0, 238.0 / 255.0, 238.0 / 255.0);
pub const FOREGROUND: Color = Color::from_rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0);
pub const HOVERED: Color = Color::from_rgb(129.0 / 255.0, 129.0 / 255.0, 129.0 / 255.0);
pub const ACCENT: Color = Color::from_rgb(15.0 / 255.0, 86.0 / 255.0, 179.0 / 255.0);
// pub const SUCCESS: Color = Color::from_rgb(31.0 / 255.0, 139.0 / 255.0, 36.0 / 255.0);
// pub const WARNING: Color = Color::from_rgb(212.0 / 255.0, 176.0 / 255.0, 17.0 / 255.0);
// pub const ERROR: Color = Color::from_rgb(218.0 / 255.0, 16.0 / 255.0, 11.0 / 255.0);

pub enum CustomButton {
    Default,
    Sidebar,
    SelectedSidebar,
}

impl button::StyleSheet for CustomButton {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: match self {
                CustomButton::SelectedSidebar => ACCENT,
                _ => Color::BLACK,
            },
            background: Some(Background::Color(match self {
                CustomButton::SelectedSidebar => {
                    Color { a: 0.3, ..ACCENT }
                },
                CustomButton::Sidebar => Color::TRANSPARENT,
                _ => Color::WHITE,
            })),
            border_radius: match self {
                CustomButton::Sidebar | CustomButton::SelectedSidebar => 7.0,
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
            CustomButton::Sidebar => button::Style {
                background: Some(Color { a: 0.3, ..HOVERED }.into()),
                ..active
            },
            _ => active
        }
    }
}

pub enum CustomContainer {
    Background,
    ForegroundWhite,
    ForegroundGray,
}

impl container::StyleSheet for CustomContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(match self {
               CustomContainer::Background => BACKGROUND,
               CustomContainer::ForegroundWhite => Color::WHITE,
               CustomContainer::ForegroundGray => FOREGROUND,
            })),
            border_radius: match self {
                CustomContainer::ForegroundGray => 7.0,
                _ => 0.0
            },
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
            ..container::Style::default()
        }
    }
}