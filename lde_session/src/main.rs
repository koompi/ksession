mod cli;
mod modmg;
mod sessionapp;
mod sm_xdg;
mod wmmanager;
use clap::{App, Arg};
use sessionapp::SessionApplication;
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
    let mut session = SessionApplication::new();
    session.startup();
    session.exec();
}
