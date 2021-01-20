mod general_page;
mod default_app_page;
mod auto_start;
mod env_advanced;

use general_page::{GeneralPage, GeneralMsg};
use default_app_page::{DefaultAppPage, DefaultAppMsg};
use auto_start::{AutostartPage, AutostartMsg};
use env_advanced::{EnvPage, EnvMsg};
use iced::{
   Element,
};
use config::Config;

pub struct Pages {
   pages: Vec<Page>,
   current: usize,
}

pub enum Page {
   General { general_page: GeneralPage },
   DefaultApp { default_app_page: DefaultAppPage },
   AutoStart { auto_start_page: AutostartPage },
   Env { env_page: EnvPage },
}

#[derive(Debug, Clone)]
pub enum PagesMessage {
   GeneralMessage(GeneralMsg),
   DefaultAppMessage(DefaultAppMsg),
   AutoStartMessage(AutostartMsg),
   EnvMessage(EnvMsg),
}

impl Pages {
   pub fn new(config: &Config) -> Self {
      use Page::*;
       
      Self {
         pages: vec![
            General{ general_page: GeneralPage::new(config.clone()) },
            DefaultApp{ default_app_page: DefaultAppPage::new() },
            AutoStart{ auto_start_page: AutostartPage::new() },
            Env{ env_page: EnvPage::new(config.clone()) },
         ],
         current: 0
      }
   }

   pub fn set_current(&mut self, idx: usize) {
      self.current = idx;
   }

   pub fn update(&mut self, msg: PagesMessage) {
      self.pages[self.current].update(msg);
   }

   // pub fn subscription(&self) -> Subscription<PagesMessage> {
   //    self.pages[self.current].subscription()
   // }

   pub fn view(&mut self) -> Element<PagesMessage> {
      self.pages[self.current].view()
   }

   pub fn title(&self) -> &str {
      self.pages[self.current].title()
   }
}

impl Page {
   fn update(&mut self, msg: PagesMessage) {
      use Page::*;
      use PagesMessage::*;
      match msg {
         GeneralMessage(msg) => {
            if let General{ general_page } = self {
               general_page.update(msg)
            }
         },
         DefaultAppMessage(msg) => {
            if let DefaultApp{ default_app_page } = self {
               default_app_page.update(msg)
            }
         },
         AutoStartMessage(msg) => {
            if let AutoStart{ auto_start_page } = self {
               auto_start_page.update(msg)
            }
         },
         EnvMessage(msg) => {
            if let Env{ env_page } = self {
               env_page.update(msg)
            }
         }
      }
   }

   fn view(&mut self) -> Element<PagesMessage> {
      use Page::*;
      match self {
         General { general_page } => general_page.view().map(move |msg| PagesMessage::GeneralMessage(msg)),
         DefaultApp { default_app_page } => default_app_page.view().map(move |msg| PagesMessage::DefaultAppMessage(msg)),
         AutoStart { auto_start_page } => auto_start_page.view().map(move |msg| PagesMessage::AutoStartMessage(msg)),
         Env { env_page } => env_page.view().map(move |msg| PagesMessage::EnvMessage(msg)),
      }
   }

   fn title(&self) -> &str {
      use Page::*;
      match self {
         General { .. } => "General",
         DefaultApp { .. } => "Default Applications",
         AutoStart { .. } => "Autostart Applications",
         Env { .. } => "Environment Variable",
      }
   }
}