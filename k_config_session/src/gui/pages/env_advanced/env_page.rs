use iced::{
   button, Element, Row, Text, Column, Container, Length, Button,
};
use iced_custom_widget::{table, Table, TableData, TableError, TableResult};
use iced_custom_widget::{table_column, table_columns};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use super::env_entry_page::{EnvEntryPage, EnvEntryMsg};
use crate::gui::CustomButton;
use config::Config;
use freedesktop_entry_parser::parse_entry;
use std::collections::HashMap;

const browser_key: &'static str = "Environment.BROWSER";
const term_key: &'static str = "Environment.TERM";

#[derive(Debug, Clone, Default)]
pub struct EnvPage {
   ls_env: Vec<Env>,
   selected_env: Option<usize>,
   tb_env_state: table::State,
   btn_add_state: button::State,
   btn_delete_state: button::State,
   env_entry_page: EnvEntryPage,
   is_adding: bool,
   config: Config
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Env {
   name: String,
   val: String,
}

#[derive(Debug, Clone)]
pub enum EnvMsg {
   BtnAddClicked,
   BtnDeleteClicked(usize),
   EntryMsg(EnvEntryMsg)
}

impl EnvPage {
   pub fn new(config: Config) -> Self {
      Self {
         ls_env: vec![
            Env::new("BROWSER", "firefox"),
            Env::new("GTK_CSD", "0"),
            Env::new("GTK_OVER", "0"),
            Env::new("TERM", "alacritty"),
         ],
         config,
         ..Self::default()
      }
   }

   pub fn update(&mut self, msg: EnvMsg) {
      use EnvMsg::*;

      match msg {
         BtnAddClicked => self.is_adding = true,
         BtnDeleteClicked(idx) => {
            let _ = self.ls_env.remove(idx);
         },
         EntryMsg(entry_msg) => {
            use EnvEntryMsg::*;
            match entry_msg {
               BtnCancelClicked => self.is_adding = false,
               BtnOkClicked(new_name, new_val) => {
                  if let Some(selected_idx) = self.selected_env {
                     if let Some(env) = self.ls_env.get_mut(selected_idx) {
                        env.set_name(new_name.as_str());
                        env.set_val(new_val.as_str());
                     }
                  }
                  else {
                     self.ls_env.push(Env::new(new_name.as_str(), new_val.as_str()));
                  }
                  self.env_entry_page = EnvEntryPage::default();
                  self.is_adding = false;
               },
               _ => self.env_entry_page.update(entry_msg)
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
         env_entry_page,
         is_adding,
         ..
      } = self;

      if *is_adding {
         env_entry_page.view().map(move |msg| EnvMsg::EntryMsg(msg))
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

   pub fn restore_settings(&mut self) {
      let mut value = String::new();
      self.ls_env.clear();
      match parse_entry("k_config_session/config/session.toml") {
         Ok(entry) => {
            entry.section("Environment").attrs().for_each(|attr| {
               value = self.config.get_str(attr.name).unwrap_or(String::new());
               self.ls_env.insert(0, Env::new(attr.name, value.as_str()));
               // emit envVarChanged(i, value);
            });
         },
         Err(err) => println!("Error: {}", err)
      }

      if let Err(_) = self.config.get_str(browser_key) {
         // emit envVarChanged(QL1S("BROWSER"), QString());
      }

      if let Err(_) = self.config.get_str(term_key) {
         // emit envVarChanged(QL1S("TERM"), QString());
      }
   }

   pub fn save(&mut self) {
      let mut do_restart = false;
      let mut old_settings: HashMap<String, String> = HashMap::new();
      match parse_entry("k_config_session/config/session.toml") {
         Ok(entry) => {
            entry.section("Environment").attrs().for_each(|attr| {
               let value = self.config.get_str(attr.name).unwrap_or(String::new());
               old_settings.insert(attr.name.to_string(), value);
            });
         },
         Err(err) => println!("Error: {}", err)
      }
      let n_items = self.ls_env.capacity();
      for i in 0..n_items {
         let item = self.ls_env.get(i).unwrap();
         
         if let Some(old_value) = old_settings.get(&item.name) {
            if *old_value != item.val {
               do_restart = true;
            }
         }
         self.config.set(item.name.as_str(), item.val.as_str());
      }

      if old_settings.capacity() != n_items {
         do_restart = true;
      }

      if do_restart {
         // emit needRestart();
      }
   }
}

impl Env {
   pub fn new(name: &str, val: &str) -> Self {
      Self {
         name: name.to_string(),
         val: val.to_string(),
      }
   }

   pub fn set_name(&mut self, name: &str) {
      self.name = name.to_string();
   }

   pub fn set_val(&mut self, val: &str) {
      self.val = val.to_string();
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