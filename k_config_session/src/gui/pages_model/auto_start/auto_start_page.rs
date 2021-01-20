use super::{
    auto_start_model::AutostartModel, auto_start_item::AutostartItem, auto_start_utils::AutostartUtils,
    auto_start_edit_page::{AutostartEditPage, AutostartEditMsg},
};
use iced::{
    button, scrollable, Button, Scrollable,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct AutostartPage {
    model: Option<AutostartModel>,
    selected_item_key: Option<String>,
    scroll_ls_app: scrollable::State,
    btn_add_state: button::State,
    btn_edit_state: button::State,
    btn_delete_state: button::State,
    auto_start_edit_page: AutostartEditPage,
    is_editing: bool,
}

#[derive(Debug, Clone)]
pub enum AutostartMsg {
   AutoStartAppToggled(String, bool),
   AutoStartModuleExpanded(usize, bool),
   BtnAddClicked,
   BtnEditClicked(String),
   BtnDeleteClicked(String),
   EditMsg(AutostartEditMsg),
}

impl AutostartPage {
    fn restore_settings(mut self) -> Self {
        self.model = Some(AutostartModel::new());
        // updateButtons();
        self
    }

    fn save(&mut self) {
        let mut do_restart = false;
        let prev_items: HashMap<String, AutostartItem> = AutostartItem::create_item_map().into_iter().filter(|(key, item)| AutostartUtils::is_k_module(item.file())).collect();
        if let Some(current_model) = self.model {
            let current_items: HashMap<String, AutostartItem> = current_model.items().into_iter().filter(|(key, item)| AutostartUtils::is_k_module(item.file())).collect();
            if prev_items.len() != current_items.len() {
                do_restart = true;
            } else {
                for (key, item) in current_model.items() {
                    if prev_items.contains_key(&key) {
                        if item.file() != prev_items.get(&key).unwrap().file() {
                            do_restart = true;
                            break;
                        }
                    } else {
                        do_restart = true;
                        break;
                    }
                }
            }

            if do_restart {
                // emit needRestart();
            }
    
            current_model.write_changes();    
        }
    }
}

impl AutostartPage {
    pub fn new() -> Self {
        let auto_start_page = Self {
            ..Self::default()
        };
        auto_start_page.restore_settings()
    }

    pub fn update(&mut self, msg: AutostartMsg) {
        use AutostartMsg::*;
        match msg {
            AutoStartAppToggled(key, is_checked) => {
                if let Some(model) = self.model {
                    if let Some(selected_item) = model.items().get(&key) {

                    }
                }
            },
            BtnAddClicked => self.is_editing = true,
            BtnEditClicked(key) => {
                if let Some(model) = self.model {
                    if let Some(selected_item) = model.items().get_mut(&key) {
                        selected_item
                    }
                }
            }
        }
    }
}