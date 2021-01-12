use std::collections::HashMap;
use std::path::PathBuf;
use xdg::BaseDirectories;

#[derive(Debug, Clone)]
pub struct AutostartItem {
   system_file: Option<PathBuf>,
   system: bool,
   local_file: Option<PathBuf>,
   local_state: FileState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileState {
   StateNone,      
   StateDeleted,   
   StateTransient, 
   StateModified,  
   StateExists     
}

impl AutostartItem {
   pub fn new() -> Self {
      Self {
         system_file: None,
         system: false,
         local_file: None,
         local_state: FileState::StateNone,
      }
   }

   pub fn with_system_file(mut self, file: &PathBuf) -> Self {
      self.system_file = Some(file.clone());
      self.system = true;
      self
   }

   pub fn set_local_from_file(&mut self, file: &PathBuf) {
      self.local_file = Some(file.clone());
      self.local_state = FileState::StateExists;
   }

   pub fn systemfile(&self) -> Option<PathBuf> {
      self.system_file.clone()
   }

   pub fn file(&self) -> PathBuf {
      if self.is_local() {
         self.system_file.clone().unwrap()
      } else {
         self.local_file.clone().unwrap()
      }
   }

   pub fn name(&self) -> &str {
      self.file().file_name().unwrap().to_str().unwrap()
   }

   pub fn set_file(&mut self, file: &PathBuf) {
      let local = self.is_local();
      if let Some(system_file) = self.system_file {
         if self.system && local && *file == system_file {
            self.remove_local();
         }
      } else {
         if local {
            self.local_state = FileState::StateModified;
         } else {
            self.local_state = FileState::StateTransient;
         }
         self.local_file = Some(file.clone())
      }
   }

   pub fn commit(&self) -> bool {
      if self.local_state == FileState::StateDeleted {
         self.local_state = FileState::StateNone;
         if let Some(local_file) = self.local_file {
            match std::fs::remove_file(local_file) {
               Ok(_) => return true,
               Err(_) => return false,
            }
         }
      } else if self.local_state == FileState::StateModified || self.local_state == FileState::StateTransient {
         self.local_state = FileState::StateExists;
      }
      true
   }

   pub fn remove_local(&self) -> bool {
      if !self.is_local() {
         return false;
      }

      self.local_state = if self.local_state == FileState::StateTransient {
         FileState::StateNone
      } else {
         FileState::StateDeleted
      };

      return !self.system;
   }

   pub fn overrides(&self) -> bool {
      self.system && self.is_local()
   }

   pub fn is_local(&self) -> bool {
      return self.local_state != FileState::StateNone && self.local_state != FileState::StateDeleted
   }

   pub fn is_empty(&self) -> bool {
      return !self.system && self.local_state == FileState::StateNone
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
               items.insert(
                  name.to_str().unwrap().to_string(),
                  AutostartItem::new().with_system_file(file),
               );
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
