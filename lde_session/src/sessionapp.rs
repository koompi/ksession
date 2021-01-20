#![allow(dead_code)]
use crate::modmg::{LDEModuleManager, ModuleManager};
#[derive(Default)]
pub struct SessionApplication {
    modmg: ModuleManager,
    config_name: String,
}
impl SessionApplication {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn set_windowmanager(&mut self, name: &str) {
        self.modmg.set_window_manager(name);
    }
    fn set_config(&self) {}
    pub async fn startup(&mut self) {
        self.set_windowmanager("kwin_x11");
        self.modmg.startup();
        self.modmg.start_autostart().await;
    }

    fn load_enviromentsettings(&mut self) {}
    fn load_keyboardsettings(&mut self) {}
    fn load_mousesettings(&mut self) {}
}
