mod cli;
mod modmg;
mod sm_xdg;
mod wmmanager;
use clap::{App, Arg};
fn main() {
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
    match matches.value_of("find") {
        Some(val) => {
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
        None => {}
    }
    let mut data = modmg::ModuleManager::new();
    data.set_window_manager("kwin_x11");
    data.start_wm();
    // data.start_process("lxqt-panel");
}
