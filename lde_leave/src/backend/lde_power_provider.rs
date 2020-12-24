use super::{helpers::{dbus_call, dbus_call_init_system}, lde_power::Action};
use dbus::{Error, blocking::Connection};
use std::time::Duration;

const INIT_SYS_SERVICE: &'static str = "org.freedesktop.PowerManagement";
const INIT_SYS_PATH: &'static str = "/org/freedesktop/PowerManagement";
const INIT_SYS_INTERFACE: &'static str = "org.freedesktop.PowerManagement";

const LDE_SERVICE: &'static str = "org.kde.ksmserver";
const LDE_PATH: &'static str = "/KSMServer";
const LDE_INTERFACE: &'static str = "org.kde.KSMServerInterface";

pub trait LdePowerProvider {
    // fn new() -> Self;
    
    fn can_action(&self, action: Action) -> Result<bool, Error>;

    fn do_action(&self, action: Action) -> Result<bool, Error>;
}

pub struct InitSystemProvider;

impl LdePowerProvider for InitSystemProvider {
    fn can_action(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerReboot => "CanReboot",
            Action::PowerShutdown => "CanPowerOff",
            Action::PowerSuspend => "CanSuspend",
            Action::PowerHibernate => "CanHibernate",
            _ => return Ok(false),
        };

        let conn = Connection::new_system()?;
        dbus_call_init_system(INIT_SYS_SERVICE, INIT_SYS_PATH, INIT_SYS_INTERFACE, &conn, cmd, false, Duration::from_millis(5000))
    }

    fn do_action(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerReboot => "Reboot",
            Action::PowerShutdown => "PowerOff",
            Action::PowerSuspend => "Suspend",
            Action::PowerHibernate => "Hibernate",
            _ => return Ok(false),
        };

        let conn = Connection::new_system()?;
        dbus_call_init_system(INIT_SYS_SERVICE, INIT_SYS_PATH, INIT_SYS_INTERFACE, &conn, cmd, true, Duration::from_millis(5000))
    }
}

pub struct LdeProvider;

impl LdePowerProvider for LdeProvider {
    fn can_action(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerLogout => "canLogout",
            Action::PowerReboot => "canReboot",
            Action::PowerShutdown => "canPowerOff",
            _ => return Ok(false),
        };
        
        let conn = Connection::new_session()?;
        dbus_call(LDE_SERVICE, LDE_PATH, LDE_INTERFACE, &conn, cmd, None, Duration::from_millis(5000))
    }

    fn do_action(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerLogout => "logout",
            Action::PowerReboot => "reboot",
            Action::PowerShutdown => "poweroff",
            _ => return Ok(false),
        };

        let conn = Connection::new_session()?;
        dbus_call(LDE_SERVICE, LDE_PATH, LDE_INTERFACE, &conn, cmd, None, Duration::from_millis(5000))
    }
}