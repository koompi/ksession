use super::pages::{Pages, PagesMessage};
use super::tab::{Tab, TabMsg};
use super::constants::{IMAGE_PATH};
use super::styles::{CustomButton, CustomContainer};
use iced::{
   button, executor, scrollable, window, Application, Button, Command, Container, Element, Length, Row, 
   Scrollable, Settings, Text, Column, 
};

pub struct LdeSessionManager {
   tabs: Vec<Tab>,
   selected_tab: usize,
   pages: Pages,
   btn_default_state: button::State,
   sidebar_scroll: scrollable::State,
}

impl LdeSessionManager {
   pub fn init() {
      // let image = image::open(format!("{}/assets/images/icon.png", ROOT_PATH()))
      //    .expect("Failed to open icon path")
      //    .into_rgba8();
      // let (width, height) = image.dimensions();
      // let rgba = image.into_raw();

      LdeSessionManager::run(Settings {
         default_text_size: 13,
         window: window::Settings {
            min_size: Some((650, 500)),
            // icon: Some(window::Icon::from_rgba(rgba, width, height).expect("Failed to open icon")),
            ..window::Settings::default()
         },
         ..Settings::default()
      }).expect("running LDE Session Manager GUI");
   }
}

impl Default for LdeSessionManager {
   fn default() -> Self {
      let tabs = vec![
         Tab::new(format!("{}/general.svg", IMAGE_PATH, ), "General Settings"),
         Tab::new(format!("{}/default_app.svg", IMAGE_PATH, ), "Default Applications"),
         Tab::new(format!("{}/startup.svg", IMAGE_PATH, ), "Autostart Applications"),
         Tab::new(format!("{}/env.svg", IMAGE_PATH, ), "Environment Variable"),
      ];

      Self {
         tabs,
         selected_tab: 0,
         pages: Pages::new(),
         btn_default_state: button::State::new(),
         sidebar_scroll: scrollable::State::new(),
      }
   }
}

#[derive(Debug, Clone)]
pub enum LdeSessionManagerMsg {
   TabMessage(usize, TabMsg),
   PagesMessage(PagesMessage),
   DefaultClicked,
}

impl Application for LdeSessionManager {
   type Executor = executor::Default;
   type Message = LdeSessionManagerMsg;
   type Flags = ();

   fn new(_flags: ()) -> (Self, Command<Self::Message>) {
      (
         Self::default(),
         Command::none()
      )
   }

   fn title(&self) -> String {
      self.pages.title().to_string()
   }

   fn update(&mut self, msg: Self::Message) -> Command<Self::Message> {
      match msg {
         Self::Message::TabMessage(idx, tab_msg) => {
            if let Some(tab) = self.tabs.get_mut(idx) {
               tab.update(tab_msg);
               self.selected_tab = idx;
               self.pages.set_current(idx);
            }
         },
         Self::Message::PagesMessage(page_msg) => {
            self.pages.update(page_msg);
         },
         Self::Message::DefaultClicked => {
            let current_tab = self.selected_tab;
            *self = Self::default();
            self.selected_tab = current_tab;
            self.pages.set_current(self.selected_tab);
         }
      }

      Command::none()
   }

   fn view(&mut self) -> Element<Self::Message> {
      let LdeSessionManager {
         tabs,
         selected_tab,
         pages,
         btn_default_state,
         sidebar_scroll,
      } = self;

      let sidebar = tabs.iter_mut().enumerate().fold(
         Scrollable::new(sidebar_scroll).spacing(10).padding(10).scroller_width(5).scrollbar_width(7), |sidebar, (idx, tab)| {
            sidebar.push(
               Container::new(tab.view(idx == *selected_tab).map(move |msg| Self::Message::TabMessage(idx, msg)),).width(Length::Units(90)).height(Length::Units(90))
            )
         },
      );
      let sidebar_sec = Container::new(sidebar).width(Length::Units(110)).height(Length::Fill).center_x().style(CustomContainer::Background);

      let content = pages.view().map(Self::Message::PagesMessage);
      let content_sec = Container::new(content).width(Length::Fill).height(Length::Fill).padding(10).style(CustomContainer::Background);

      let btn_default = Button::new(btn_default_state, Text::new("   Default   ")).on_press(Self::Message::DefaultClicked).style(CustomButton::Default);

      Container::new(
         Column::new().spacing(15)
         .push(
            Row::new().spacing(27).height(Length::Fill)
            .push(sidebar_sec)
            .push(content_sec)
         )
         .push(btn_default)
      ).padding(20).into()
   }
}
