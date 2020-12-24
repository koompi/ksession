
use structopt::{StructOpt, clap};
use crate::backend::{Action, LdePower, print_dbus_msg};

/// Easily triggering leave session actions: logout, reboot, shutdown, hibernate, and lock screen
#[derive(StructOpt, Debug)]
#[structopt(name = "LDE Leave")]
pub struct LeaveOpts {
    /// Switch User.
    #[structopt(long)]
    switchuser: bool,

    /// Logout.
    #[structopt(long)]
    logout: bool,

    /// Lockscreen.
    #[structopt(long)]
    lockscreen: bool,

    /// Suspend.
    #[structopt(long)]
    suspend: bool,

    /// Hibernate.
    #[structopt(long)]
    hibernate: bool,

    /// Shutdown.
    #[structopt(long)]
    shutdown: bool,

    /// Reboot.
    #[structopt(long)]
    reboot: bool,
}

pub fn get_opts() -> Result<LeaveOpts, clap::Error> {
    LeaveOpts::from_args_safe()
}

pub fn validate_opts_or_exit(
    opts_result: Result<LeaveOpts, clap::Error>,
    power_manager: &LdePower,
) -> bool {
    let mut flag = true;

    match opts_result {
        Ok(opt) => {
            if opt.logout {
                match power_manager.do_action(Action::PowerLogout) {
                    Ok(res) => {
                        flag = false;
                        println!("Logout: {}", res);
                    }
                    Err(err) => print_dbus_msg(err)
                }
            }
        
            if opt.suspend {
                match power_manager.do_action(Action::PowerSuspend) {
                    Ok(res) => {
                        flag = false;
                        println!("Suspend: {}", res);
                    }
                    Err(err) => print_dbus_msg(err)
                }
            }
        
            if opt.hibernate {
                match power_manager.do_action(Action::PowerHibernate) {
                    Ok(res) => {
                        flag = false;
                        println!("Hibernate: {}", res);
                    }
                    Err(err) => print_dbus_msg(err)
                }
            }
        
            if opt.shutdown {
                match power_manager.do_action(Action::PowerShutdown) {
                    Ok(res) => {
                        flag = false;
                        println!("Shut down: {}", res);
                    }
                    Err(err) => print_dbus_msg(err)
                }
            }
        
            if opt.reboot {
                match power_manager.do_action(Action::PowerReboot) {
                    Ok(res) => {
                        flag = false;
                        println!("Reboot: {}", res);
                    }
                    Err(err) => print_dbus_msg(err)
                }
            }
            flag
        },
        Err(e) => {
            e.exit();
        }
    }
}