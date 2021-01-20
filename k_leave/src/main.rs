mod cli;
mod gui;

use cli::{get_opts, validate_opts_or_exit};
use gui::KLeave;
use libkoompi::session::PowerManager;

fn main() {
    let power_manager = PowerManager::new();
    let opts_result = get_opts();
    let flag = validate_opts_or_exit(opts_result, &power_manager);

    if flag {
        KLeave::init(power_manager);
    }
}   