#![allow(dead_code)]
use crate::sm_xdg::xdg_autostart::desktop_files;
use std::process::Command;
use wmctrl;
pub struct ModuleManager {
    wmStarted: bool,
    mTrayStarted: bool,
    // brief Window Manager command
    window_manager: String,
}
impl ModuleManager {
    pub fn new() -> Self {
        Self {
            wmStarted: false,
            mTrayStarted: false,
            window_manager: String::from(""),
        }
    }
    /// Start a module by a given file (e.g "lde_panel.desktop")
    pub fn start_process(self, proc_name: &str) -> Self {
        self
    }
    pub fn start_autostart(&self) -> &Self {
        let files = desktop_files();
        for i in files {
            println!("{}", i);
        }
        self
    }
    /// Stop a running module
    pub fn stop_process(mut self, proc_name: &str) -> Self {
        self
    }
    /// Set the window manager (e.g "/usr/bin/kwin_x11")
    pub fn set_window_manager(&mut self, wm_name: &str) -> &mut Self {
        self.window_manager = wm_name.to_string();
        self
    }
    pub fn startup(mut self) -> Self {
        self
    }
    pub fn start_wm(&mut self) -> &mut Self {
        // we comment this out because when the window manager is empty it break the program
        // if Self::get_wm().is_empty() {
        //     self.wmStarted = true;
        //     return self;
        // }
        let mut output = Command::new(&self.window_manager)
            .arg("--replace")
            .arg("&")
            .spawn()
            .expect("kwin_x11 faile to start");
        match output.wait() {
            Ok(status) => {
                println!("stautus: {}", status);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        self
    }
}
pub fn get_wm() -> String {
    let st = String::from_utf8(wmctrl::show_wm_information().stdout).unwrap();
    let l1 = st
        .split('\n')
        .map(|f| f.to_string())
        .collect::<Vec<String>>()[0]
        .clone()
        .split(':')
        .map(|f| f.to_string())
        .collect::<Vec<String>>()[1]
        .clone()
        .chars()
        .into_iter()
        .filter(|s| !s.is_whitespace())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("");
    l1
}
