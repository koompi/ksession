#![allow(dead_code)]
#![allow(unused_variables)]
use crate::sm_xdg::xdg_autostart::desktop_files;
use async_trait::async_trait;
use std::fs;
use std::path::PathBuf;
use std::process::{ExitStatus, Stdio};
use tokio::process::Command;
#[async_trait]
pub trait LDEModuleManager {
    fn set_window_manager(&mut self, wm_name: &str);
    async fn start_process(&self, proc_name: &str);
    fn stop_process(&self, proc_name: &str);
    fn list_modlues(&mut self) -> Vec<String>;
    async fn startup(&mut self);
    fn logout(&self, can_exit: bool);
}
#[derive(Default)]
pub struct ModuleManager {
    wm_started: bool,
    tray_started: bool,
    // keep crashes for a given process to raise a message in case of repeating crashes
    is_crashed: bool,
    // brief Window Manager command
    window_manager: String,
}
#[async_trait]
impl LDEModuleManager for ModuleManager {
    fn set_window_manager(&mut self, wm_name: &str) {
        self.window_manager = wm_name.to_string();
    }
    async fn start_process(&self, proc_name: &str) {
        LDEModule::new("system_settings").start();
    }
    fn stop_process(&self, proc_name: &str) {}
    fn list_modlues(&mut self) -> Vec<String> {
        Vec::<String>::new()
    }
    async fn startup(&mut self) {
        self.start_wm().await;
    }
    fn logout(&self, can_exit: bool) {}
}

impl ModuleManager {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    async fn start_wm(&mut self) -> &mut Self {
        if !get_wm().is_empty() {
            self.wm_started = true;
            return self;
        } else {
            LDEModule::new(self.window_manager.as_str()).start();
        }
        self
    }
    pub async fn wm_started(&mut self) {
        println!("Window manager started");
        println!("window manager: {}", get_wm());
    }
    pub async fn start_autostart(&mut self) -> &mut Self {
        let list_files = desktop_files();

        for file in list_files {
            let content = fs::read_to_string(&file).expect("Something wrong reading file");
            if content.contains("X-LXQt-Module") {
                match self.start_app_process(file).await {
                    Ok(()) => println!("run fine"),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        self
    }
    async fn start_app_process(&mut self, file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let entry = freedesktop_entry_parser::parse_entry(file).unwrap();
        let binary = entry
            .section("Desktop Entry")
            .attr("Exec")
            .expect("Attribute doesn't exist");
        println!("{}", binary);
        LDEModule::new(binary).start();
        Ok(())
    }
    fn start_config_update(&mut self) {}
    fn restart_module(&self, exit_status: ExitStatus) {}
}
pub fn get_wm() -> String {
    let st = wmctrl::show_wm_information();
    if ExitStatus::success(&st.status) {
        let output = String::from_utf8(wmctrl::show_wm_information().stdout).unwrap();
        output.split('\n').collect::<Vec<&str>>()[0]
            .split(':')
            .collect::<Vec<&str>>()[1]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("[]")
    } else {
        String::from("")
    }
}

pub struct LDEModule<'l> {
    is_terminated: bool,
    // path: std::path::PathBuf,
    filename: &'l str,
}

impl<'l> LDEModule<'l> {
    pub fn new(module: &'l str) -> Self {
        Self {
            is_terminated: false,
            filename: module,
        }
    }
    pub fn start(&mut self) {
        let mut cmd = Command::new(self.filename)
            .args(["&"].iter())
            .spawn()
            .expect("File to run module");
        tokio::spawn(async move {
            match cmd.try_wait() {
                Ok(data) => println!("output : {:?}", data),
                Err(e) => println!("Error: {:?}", e),
            }
        });
    }
    pub fn terminate(&mut self) {
        self.is_terminated = true;
        std::process::exit(0x0100);
    }
    pub fn is_terminating(&self) -> bool {
        self.is_terminated
    }
}
