use super::auto_start_item::AutostartItem;
use super::auto_start_utils::AutostartUtils;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::gui::load_config;

#[derive(Debug, Clone, Default)]
pub struct AutostartModel {
    item_map: HashMap<String, AutostartItem>,
    global_items: Vec<String>,
    global_selected: (usize, usize),
    lde_items: Vec<String>,
    lde_selected: (usize, usize),
}

impl AutostartModel {
    pub fn new() -> Self {
        let item_map = AutostartItem::create_item_map();
        let mut global_items = Vec::new();
        let mut lde_items = Vec::new();
        item_map.iter().for_each(|(key, item)| {
            if !AutostartUtils::is_lde_module(item.file()) {
                if AutostartUtils::show_only_in_lde(item.file()) {
                    lde_items.push(*key);
                } else {
                    global_items.push(*key);
                }
            }
        });

        Self {
            item_map,
            global_items,
            lde_items,
            global_selected: (0, 0),
            lde_selected: (1, 0), 
        }
    }

    pub fn write_changes(&self) -> bool {
        self.item_map.iter().fold(true, |is_changed, (key, item)| {
            item.commit()
        })
    }

    pub fn items(&self) -> HashMap<String, AutostartItem> {
        let mut all_items: HashMap<String, AutostartItem>  = HashMap::new();
        self.lde_items.iter().for_each(|key| {
            all_items.insert(*key, *self.item_map.get(key).unwrap());
        });
        self.global_items.iter().for_each(|key| {
            all_items.insert(*key, *self.item_map.get(key).unwrap());
        });
        all_items
    }

    pub fn set_entry(&mut self, idx: (usize, usize), file: &PathBuf, overwrite: bool) -> bool {
        let file_name = file.file_name().unwrap().to_str().unwrap();

        let replacing = self.item_map.contains_key(file_name);
        if !overwrite && replacing {
            return false;
        }

        if idx == self.lde_selected {
            let file_config = load_config(Some(file));
            match file_config.get_str("OnlyShowIn") {
                Ok(current_val) => {
                    if !current_val.contains("LDE;") {
                        file_config.set("OnlyShowIn", format!("{}{}", current_val, "LDE;"));
                    }
                }, 
                Err(_) => {
                    file_config.set("OnlyShowIn", "LDE;");
                }
            }
        }
        if let Some(selected_item) = self.item_map.get_mut(file_name) {
            selected_item.set_file(file);
        }
        if replacing {
            // emit dataChanged(index, index);
            return true;
        }
        if idx == self.global_selected {
            self.global_items.push(file_name.to_owned());
        } else {
            self.lde_items.push(file_name.to_owned());
        }

        true
    }

    pub fn set_data(&mut self, idx: (usize, usize), value: AutostartItem) -> bool {
        let mut name = String::new();
        if idx == self.global_selected {
            if let Some(val) = self.global_items.get(idx.1) {
                name = *val;
            } 
        }
        else if let Some(val) = self.lde_items.get(idx.1) {
            name = *val;
        }
        // emit dataChanged(index, index);
        true
    }

    pub fn remove_row(&mut self, row: usize, idx: (usize, usize)) -> bool {
        let mut item = String::new();
        if idx == self.global_selected {
            if let Some(selected_key) = self.global_items.get(row) {
                item = *selected_key;
            }
        } else if let Some(selected_key) = self.lde_items.get(row) {
            item = *selected_key;
        }

        if let Some(selected_item) = self.item_map.get(&item) {
            if !selected_item.remove_local() {
                // emit dataChanged(index, index);
                return false;
            }
        }

        if idx == self.global_selected {
            self.global_items.remove(row);
        } else {
            self.lde_items.remove(row);
        }
        if let Some(selected_item) = self.item_map.get(&item) {
            if selected_item.is_empty() {
                let _ = self.item_map.remove(&item);
            }
        }
        true
    }
}
