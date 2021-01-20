#![allow(dead_code)]
#![allow(unused_variables)]
use crate::sm_xdg::xdg_autostart::desktop_files;
use std::fs;
use std::path::PathBuf;
use std::process::ExitStatus;
use tokio::process::Command;
pub trait LDEModuleManager {
    fn set_window_manager(&mut self, wm_name: &str);
    fn start_process(&self, proc_name: &str);
    fn stop_process(&self, proc_name: &str);
    fn list_modlues(&mut self) -> Vec<String>;
    fn startup(&mut self);
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

impl LDEModuleManager for ModuleManager {
    fn set_window_manager(&mut self, wm_name: &str) {
        self.window_manager = wm_name.to_string();
    }
    fn start_process(&self, proc_name: &str) {
        match Command::new(proc_name).spawn() {
            Ok(mut child) => {
                tokio::spawn(async move {
                    let status = child
                        .wait()
                        .await
                        .expect("child process encountered an error");

                    println!("child status was: {}", status);
                });
            }
            Err(e) => println!("error: {:?}", e),
        }
    }
    fn stop_process(&self, proc_name: &str) {}
    fn list_modlues(&mut self) -> Vec<String> {
        Vec::<String>::new()
    }
    fn startup(&mut self) {
        self.start_wm();
    }
    fn logout(&self, can_exit: bool) {}
}

impl ModuleManager {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn start_wm(&mut self) {
        if !get_wm().is_empty() {
            self.wm_started = true;
        } else {
            self.start_process(&self.window_manager);
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    pub fn wm_started(&mut self) {
        println!("window manager: {}", get_wm());
    }
    pub fn start_autostart(&mut self) {
        let list_files = desktop_files();

        for file in list_files {
            let content = fs::read_to_string(&file).expect("Something wrong reading file");
            if content.contains("X-LDE-Module") {
                self.start_app_process(file);
            }
        }
    }
    fn start_app_process(&mut self, file: PathBuf) {
        match freedesktop_entry_parser::parse_entry(file) {
            Ok(entry) => match entry.section("Desktop Entry").attr("Exec") {
                Some(binary) => {
                    self.start_process(binary);
                }
                None => {
                    eprintln!("Binary not found")
                }
            },
            Err(e) => {
                eprintln!("failed to parse desktop file: ")
            }
        }
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
