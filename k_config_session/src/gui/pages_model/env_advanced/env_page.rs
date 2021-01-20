use config::Config;
use iced::{button, Button, Element, Container, Column, Row, Text, Length};
use iced_custom_widget::{table, table_columns, Table, TableData, TableResult, TableError};
use super::env_edit_page::{EnvEditPage, EnvEditMsg};
use crate::gui::CustomButton;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use freedesktop_entry_parser::parse_entry;
use std::collections::HashMap;

const browser_key: &'static str = "Environment.BROWSER";
const term_key: &'static str = "Environment.TERM";

#[derive(Debug, Clone, Default)]
pub struct EnvPage {
   config: Config,
   btn_add_state: button::State,
   btn_delete_state: button::State,
   ls_env: Vec<Env>,
   selected_env: Option<usize>,
   tb_env_state: table::State,
   is_adding: bool,
   env_edit_page: EnvEditPage
}

#[derive(Debug, Clone)]
pub enum EnvMsg {
   BtnAddClicked,
   BtnDeleteClicked(usize),
   EditMsg(EnvEditMsg),
}

impl EnvPage {
   fn restore_settings(mut self) -> Self {
      let mut val = String::new();
      self.ls_env.clear();
      match parse_entry("config/session.toml") {
         Ok(entry) => entry.section("Environment").attrs().for_each(|attr| {
            val = self.config.get_str(attr.name).unwrap_or(String::new());
            self.ls_env.insert(0, Env::new(attr.name, val.as_str()));
            self.update_item(attr.name, val.as_str());
         }),
         Err(err) => println!("Error: {}", err)
      }

      if let Err(_) = self.config.get_str(browser_key) {
         // self.config.set(browser_key, String::new());
         self.update_item(browser_key, "");
      }
      if let Err(_) = self.config.get_str(term_key) {
         // self.config.set(term_key, String::new());
         self.update_item(term_key, "");
      }
      self
   }

   pub fn save(&mut self) {
      let mut do_restart = false;
      let mut old_settings: HashMap<&str, &str> = HashMap::new();
      match parse_entry("config/session.toml") {
         Ok(entry) => entry.section("Environment").attrs().for_each(|attr| {
            old_settings.insert(attr.name, attr.value.unwrap_or(""));
         })
      }
      self.ls_env.iter().for_each(|env| {
         if let Some(old_env_val) = old_settings.get(env.name.as_str()) {
            if *old_env_val != env.val {
               do_restart = true;
            }
            self.config.set(env.name.as_str(), env.val);
         }
      });

      if old_settings.capacity() != self.ls_env.len() {
         do_restart = true;
      }

      if do_restart {
         // emit needRestart();
      }
   }

   fn update_item(&mut self, var: &str, val: &str) {
      let item_ls: Vec<(usize, &mut Env)> = self.ls_env.iter_mut().enumerate().filter(|(_, env)| env.name == var).collect();
      if item_ls.is_empty() {
         self.ls_env.insert(0, Env::new(var, val));
         return;
      }

      item_ls.iter_mut().for_each(|(idx, item)| {
         if !val.is_empty() {
            item.set_val(val);
         } else {
            let _ = self.ls_env.remove(*idx);
         }
      });
   }
}

impl EnvPage {
   pub fn new(config: Config) -> Self {
      let mut env_page = Self {
         config,
         ..Self::default()
      };
      env_page.restore_settings()
   }

   pub fn update(&mut self, msg: EnvMsg) {
      use EnvMsg::*;
      use EnvEditMsg::*;
      match msg {
         BtnAddClicked => self.is_adding = true,
         BtnDeleteClicked(idx) => {
            let item = self.ls_env.remove(idx);
            self.update_item(item.name.as_str(), "");
         },
         EditMsg(edit_msg) => match edit_msg {
            BtnCancelClicked => self.is_adding = false,
            BtnOkClicked(new_name, new_val) => {
               if let Some(selected_idx) = self.selected_env {
                  if let Some(item) = self.ls_env.get(selected_idx) {
                     self.update_item(new_name.as_str(), new_val.as_str());
                  }
               } else {
                  self.ls_env.insert(0, Env::new(new_name.as_str(), new_val.as_str()));
               }
            }
         }
      }
   }

   pub fn view(&mut self) -> Element<EnvMsg> {
      let EnvPage {
         ls_env,
         selected_env,
         tb_env_state,
         btn_add_state,
         btn_delete_state,
         env_edit_page,
         is_adding,
         ..
      } = self;

      if *is_adding {
         env_edit_page.view().map(move |msg| EnvMsg::EditMsg(msg))
      } else {
         let txt_env = Text::new("Environment Variables").size(14);
         let tb_columns = table_columns![("name", "Name"), ("val", "Value"),];
         let tb_ls_env = Table::new(tb_env_state, tb_columns, ls_env).width(Length::Fill);
         let btn_add = Button::new(btn_add_state, Text::new("  Add  ")).on_press(EnvMsg::BtnAddClicked).style(CustomButton::Default);
         let mut btn_delete = Button::new(btn_delete_state, Text::new("  Delete  ")).style(CustomButton::Default);
         if let Some(selected_idx) = *selected_env {
            btn_delete = btn_delete.on_press(EnvMsg::BtnDeleteClicked(selected_idx));
         }
         let btn_group = Column::new().spacing(10)
            .push(btn_add)
            .push(btn_delete);
   
         Container::new(
            Column::new().spacing(10)
            .push(txt_env)
            .push(
               Row::new().spacing(10)
               .push(tb_ls_env)
               .push(btn_group)
            )
         ).into()
      }
   }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Env {
   name: String,
   val: String,
}

impl Env {
   pub fn new(name: &str, val: &str) -> Self {
      Self {
         name: name.to_owned(),
         val: val.to_owned(),
      }
   }

   pub fn set_val(&mut self, val: &str){
      self.val = val.to_owned();
   } 
}

impl TableData for Env {
   fn get_field_value(&self, field_name: &str) -> TableResult<Value> {
      let value = match field_name {
         "name" => serde_json::to_value(&self.name),
         "val" => serde_json::to_value(&self.val),
         s => return Err(TableError::InvalidFieldName(s.to_owned())),
      };
      Ok(value.unwrap())
   }
}