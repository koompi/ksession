use iced::{
    button, container, pick_list, checkbox, Background, Color, Vector,
};
use iced_style::menu;

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
    Text,
    SelectedText,
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
                CustomButton::Default => Color::WHITE,
                CustomButton::SelectedText => BACKGROUND,
                _ => Color::TRANSPARENT,
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

        button::Style {
            background: match self {
                CustomButton::Sidebar => Color { a: 0.3, ..HOVERED }.into(),
                CustomButton::Default => BACKGROUND.into(),
                CustomButton::Text => BACKGROUND.into(),
                _ => active.background
            },
            ..active
        }
    }
}

pub enum CustomContainer {
    Default,
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
               CustomContainer::Default => Color::TRANSPARENT,
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

pub enum CustomSelect {
    Default,
}
 
impl pick_list::StyleSheet for CustomSelect {
    fn menu(&self) -> menu::Style {
        let default: menu::Style = Default::default();
        menu::Style {
            selected_background: match self {
                CustomSelect::Default => ACCENT.into(),
            },
            ..default
        }
    }

    fn active(&self) -> pick_list::Style {
        match self {
            CustomSelect::Default => pick_list::Style {
                text_color: Color::BLACK,
                background: Color { a: 0.3, ..ACCENT }.into(),
                icon_size: 0.5,
                border_color: ACCENT,
                border_radius: 5.0,
                border_width: 0.0,
            },
        }
    }
    fn hovered(&self) -> pick_list::Style {
        let active = self.active();
    
        pick_list::Style {
            background: match self {
                CustomSelect::Default => active.background,
            },
            ..active
        }
    }
 }
pub enum CustomCheckbox {
    Default,
}
 
impl checkbox::StyleSheet for CustomCheckbox {
    fn active(&self, is_checked: bool) -> checkbox::Style {
    checkbox::Style {
        background: if is_checked { ACCENT } else { Color::WHITE }.into(),
        checkmark_color: Color::WHITE,
        border_radius: 5.0,
        border_width: 1.5,
        border_color: if is_checked { ACCENT } else { HOVERED }.into(),
    }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        self.active(is_checked)
    }
}