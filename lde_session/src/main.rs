mod cli;
mod modmg;
mod sessionapp;
mod sm_xdg;
mod wmmanager;
use clap::{App, Arg};
use modmg::{LDEModuleManager, ModuleManager};
#[tokio::main]
async fn main() {
    let matches = App::new("LDE SESSION")
        .version("1.0")
        .author("KOOMPI. koompi@gmail.com")
        .about("Sesssion Manager. ")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("--config")
                .value_name("FILE")
                .help("Configuration file path.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("find")
                .short("-f")
                .long("--find")
                .value_name("program")
                .help("Find a program in a system"),
        )
        .arg(
            Arg::with_name("wm")
                .short("-w")
                .long("--window-manager")
                .value_name("FILE")
                .help("Window manager to use."),
        )
        .get_matches();
    if let Some(val) = matches.value_of("find") {
        println!("find program: {}", val);
        let status = if wmmanager::find_program(val) {
            println!("found program ");
            true
        } else {
            println!("program not found");
            false
        };
        println!("Status: {}", status);
    }
    let mut mg = ModuleManager::new();
    mg.set_window_manager("kwin_x11");
    mg.startup().await;
    let _ = tokio::join!(mg.startup());
    mg.start_process("lxqt-panel");
    mg.start_process("desktop_manager");
    std::thread::park();
}
