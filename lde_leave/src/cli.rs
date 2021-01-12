
use structopt::{StructOpt, clap};
use crate::backend::{Action, LdePower, print_dbus_msg};

/// Easily triggering leave session actions: logout, reboot, shutdown, hibernate, and lock screen
#[derive(StructOpt, Debug)]
#[structopt(name = "LDE Leave")]
pub struct LeaveOpts {
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
                if let Err(err) = power_manager.do_action(Action::PowerLogout) {
                    print_dbus_msg(err);
                }
                flag = false;
            }
        
            if opt.suspend {
                if let Err(err) = power_manager.do_action(Action::PowerSuspend) {
                    print_dbus_msg(err);
                }
                flag = false;
            }
        
            if opt.hibernate {
                if let Err(err) = power_manager.do_action(Action::PowerHibernate) {
                    print_dbus_msg(err);
                }
                flag = false;
            }
        
            if opt.shutdown {
                if let Err(err) = power_manager.do_action(Action::PowerShutdown) {
                    print_dbus_msg(err);
                }
                flag = false;
            }
        
            if opt.reboot {
                if let Err(err) = power_manager.do_action(Action::PowerReboot) {
                    print_dbus_msg(err);
                }
                flag = false;
            }
            flag
        },
        Err(e) => {
            e.exit();
        }
    }
}