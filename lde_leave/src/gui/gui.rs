use super::{
    action_button::{ActionButton, ActionMsg, ActionType}, constants::IMAGE_PATH, styles::CustomButton,

};
use crate::backend::LdePower; 
use num_traits::FromPrimitive;
use std::path::PathBuf;
use std::ops::SubAssign;
use std::time::{Duration, Instant};
use iced::{
    button, Align, Application, Button, Column, Command, Container, Element, Length, Row, Settings, 
    Subscription, Text, time, Svg, window, HorizontalAlignment,
};

pub struct LdeLeave {
    power_manager: LdePower,
    userprofile: PathBuf,
    username: &'static str,
    action_buttons: Vec<ActionButton>,
    selected_action: usize,
    btn_ok_state: button::State,
    btn_cancel_state: button::State,
    shutdown_count_dur: Duration,
    err_msg: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    OnActionClicked(usize, ActionMsg),
    OnOkayClicked,
    OnCancelClicked,
}

impl LdeLeave {
    pub fn init(power_manager: LdePower) {
        let settings = Settings {
            antialiasing: true,
            default_text_size: 13,
            flags: power_manager,
            ..Settings::default()
        };

        LdeLeave::run(settings).expect("running LDE Session Leave GUI");
    }
}

impl Application for LdeLeave {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = LdePower;

    fn new(power_manager: LdePower) -> (Self, Command<Message>) {
        (
            Self {
                power_manager,
                username: env!("USER"),
                userprofile: format!("{}/user.svg", IMAGE_PATH).into(),
                action_buttons: vec![
                    ActionButton::new(format!("{}/sleep.svg", IMAGE_PATH), "Sleep", ActionType::Sleep),
                    ActionButton::new(format!("{}/hibernate.svg", IMAGE_PATH), "Hibernate", ActionType::Hibernate),
                    ActionButton::new(format!("{}/restart.svg", IMAGE_PATH), "Restart", ActionType::Restart),
                    ActionButton::new(format!("{}/shut-down.svg", IMAGE_PATH), "Shut Down", ActionType::Shutdown),
                    ActionButton::new(format!("{}/logout.svg", IMAGE_PATH), "Logout", ActionType::Logout),
                ],
                selected_action: 3,
                btn_ok_state: button::State::new(),
                btn_cancel_state: button::State::new(),
                shutdown_count_dur: Duration::from_secs(30),
                err_msg: None
            },
            Command::none()
        )
    }

    fn title(&self) -> String { 
        String::from("LDE Session Leave")   
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.shutdown_count_dur > Duration::from_secs(0) {
            time::every(Duration::from_millis(500)).map(Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn update(&mut self, msg: Message) -> Command<Message> { 
        let mut cmd = Command::none();

        match msg {
            Message::Tick(_) => self.shutdown_count_dur.sub_assign(Duration::from_millis(500)),
            Message::OnActionClicked(idx, msg) => {
                if let Some(action_button) = self.action_buttons.get_mut(idx) {
                    action_button.update(msg);
                    self.selected_action = idx;
                }

                cmd = Command::perform(async {}, |_| Message::OnOkayClicked);
            },
            Message::OnOkayClicked => {
                match FromPrimitive::from_usize(self.selected_action) {
                    Some(ActionType::Sleep) => {
                        match self.power_manager.suspend() {
                            Ok(_) => self.err_msg = None,
                            Err(err) => self.err_msg = err.message().map(ToString::to_string)
                        }
                    },
                    Some(ActionType::Hibernate) => {
                        match self.power_manager.hibernate() {
                            Ok(_) => self.err_msg = None,
                            Err(err) => self.err_msg = err.message().map(ToString::to_string)
                        }
                    },
                    Some(ActionType::Restart) => {
                        match self.power_manager.reboot() {
                            Ok(_) => self.err_msg = None,
                            Err(err) => self.err_msg = err.message().map(ToString::to_string)
                        }
                    },
                    Some(ActionType::Shutdown) => {
                        match self.power_manager.shutdown() {
                            Ok(_) => self.err_msg = None,
                            Err(err) => self.err_msg = err.message().map(ToString::to_string)
                        }
                    },
                    Some(ActionType::Logout) => {
                        match self.power_manager.logout() {
                            Ok(_) => self.err_msg = None,
                            Err(err) => self.err_msg = err.message().map(ToString::to_string)
                        }
                    },
                    None => self.err_msg = Some(String::from("other action"))
                }
                self.shutdown_count_dur = Duration::from_secs(0)
            }
            Message::OnCancelClicked => std::process::exit(0)
        }
        cmd
    }

    fn view(&mut self) -> Element<Message> { 
        let LdeLeave {
            userprofile,
            username,
            action_buttons,
            selected_action,
            btn_ok_state,
            btn_cancel_state,
            shutdown_count_dur,
            err_msg,
            ..
        } = self;

        let user_profile = Svg::from_path(&userprofile).width(Length::Units(150)).width(Length::Units(150));

        let user_name = Text::new(*username);
        let user_con = Container::new(
            Column::new().spacing(20).align_items(Align::Center)
            .push(user_profile)
            .push(user_name)
        ).padding(5);

        let actions_row = action_buttons.iter_mut().enumerate().fold(Row::new().spacing(30), |row, (idx, action_button)| {
            row.push(
                Container::new(
                    action_button.view(*selected_action == idx).map(move |msg| Message::OnActionClicked(idx, msg))
                ).width(Length::Units(85)).height(Length::Units(85)).center_x().center_y()
            )
        });

        let state_text = if let Some(err_msg) = err_msg.clone() {
            err_msg
        } else if *shutdown_count_dur != Duration::from_secs(0) {
            format!("Shuting down in {} seconds", shutdown_count_dur.as_secs())
        } else {
            String::new()
        };

        let txt_state = Text::new(state_text).size(14);
        let btn_ok = Button::new(btn_ok_state, Text::new("Ok").horizontal_alignment(HorizontalAlignment::Center)).width(Length::Units(100)).on_press(Message::OnOkayClicked).style(CustomButton::Default);
        let btn_cancel = Button::new(btn_cancel_state, Text::new("Cancel").horizontal_alignment(HorizontalAlignment::Center)).width(Length::Units(100)).on_press(Message::OnCancelClicked).style(CustomButton::Default);
        let btn_group = Row::new().spacing(10).align_items(Align::Center)
            .push(btn_ok)
            .push(btn_cancel);

        Container::new(
            Column::new().spacing(20).align_items(Align::Center)
            .push(user_con)
            .push(actions_row)
            .push(txt_state)
            .push(btn_group)
        ).width(Length::Fill).height(Length::Fill).center_x().center_y().into()
    }

    fn mode(&self) -> window::Mode {
        window::Mode::Fullscreen
    } 

    fn scale_factor(&self) -> f64 {
        0.85
    }

    // fn background_color(&self) -> Color {
    //     Color::from_rgba8(21, 21, 21, 0.7)
    // }
}