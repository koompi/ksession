use std::collections::HashMap;
use std::path::PathBuf;
use xdg::BaseDirectories;

pub struct AutostartItem {
   system_file: Option<PathBuf>,
   system: bool,
   local_file: Option<PathBuf>,
   local_state: FileState
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileState {
   StateTransient,
   StateExists,
}

impl AutostartItem {
   pub fn new() -> Self {
      Self {
         system_file: None,
         system: false,
         local_file: None,
         local_state: FileState::StateExists,
      }
   }

   pub fn with_system_file(file: &PathBuf) -> Self {
      Self {
         system_file: Some(file.clone()),
         system: true,
         local_file: None,
         local_state: FileState::StateExists,
      }
   }

   pub fn systemfile(&self) -> Option<PathBuf> {
      self.system_file.clone()
   }

   pub fn set_local_from_file(&mut self, file: &PathBuf) {
      self.local_file = Some(file.clone());
      self.local_state = FileState::StateExists;
   }

   // !hard_code
   pub fn remove_local(&self) -> bool {
      false
   }

   pub fn overrides(&self) -> bool {
      self.system && self.is_local()
   }

   // !hard_code
   pub fn is_local(&self) -> bool {
      false
   }

   pub fn is_transient(&self) -> bool {
      self.local_state == FileState::StateTransient
   }

   pub fn create_item_map() -> HashMap<String, AutostartItem> {
      let mut items = HashMap::new();

      BaseDirectories::with_prefix("autostart").unwrap().get_config_dirs().into_iter().for_each(|config_dir| {
         if let Ok(system_list) = list_of_desktop_files(config_dir) {
            system_list.iter().for_each(|file| {
               let name = file.file_stem().unwrap();
               items.insert(name.to_str().unwrap().to_string(), AutostartItem::with_system_file(file));
            })
         }
      });

      if let Ok(local_list) = list_of_desktop_files(BaseDirectories::with_prefix("autostart").unwrap().get_config_home()) {
         local_list.iter().for_each(|file| {
            let name = file.file_stem().unwrap();
            if items.contains_key(name.to_str().unwrap()) {
               if let Some(item) = items.get_mut(name.to_str().unwrap()) {
                  item.set_local_from_file(file);
               }
            } else {
               let mut item = AutostartItem::new();
               item.set_local_from_file(file);
               items.insert(name.to_str().unwrap().to_string(), item);
            }
         })
      }
      
      items
   }
}

fn list_of_desktop_files(dir: PathBuf) -> std::io::Result<Vec<PathBuf>> {
   let mut result = vec![];

   for path in std::fs::read_dir(dir)? {
      let path = path?.path();
      if let Some("desktop") = path.extension().and_then(std::ffi::OsStr::to_str) {
         result.push(path.to_owned());
      }
   }
   Ok(result)
}