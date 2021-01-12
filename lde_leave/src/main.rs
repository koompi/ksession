mod backend;
mod cli;
mod gui;

use cli::{get_opts, validate_opts_or_exit};
use gui::LdeLeave;
use backend::LdePower;

fn main() {
    let power_manager = LdePower::new();
    let opts_result = get_opts();
    let flag = validate_opts_or_exit(opts_result, &power_manager);

    if flag {
        LdeLeave::init(power_manager);
    }
}   