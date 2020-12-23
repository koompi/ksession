use super::{helpers::{dbus_call, dbus_call_init_system}, lde_power::Action};
use dbus::{Error, blocking::Connection};
use std::time::Duration;

const LIGHTDM_SERVICE: &'static str = "org.freedesktop.DisplayManager";
const LIGHTDM_INTERFACE: &'static str = "org.freedesktop.DisplayManager.Seat";

const INIT_SYS_SERVICE: &'static str = "org.freedesktop.PowerManagement";
const INIT_SYS_PATH: &'static str = "/org/freedesktop/PowerManagement";
const INIT_SYS_INTERFACE: &'static str = "org.freedesktop.PowerManagement";

const LDE_SERVICE: &'static str = "org.kde.ksmserver";
const LDE_PATH: &'static str = "/KSMServer";
const LDE_INTERFACE: &'static str = "org.kde.KSMServerInterface";

const PROPERTIES_INTERFACE: &'static str = "org.freedesktop.DBus.Properties";

pub trait LdePowerProvider {
    // fn new() -> Self;
    
    fn canAction(&self, action: Action) -> Result<bool, Error>;

    fn doAction(&self, action: Action) -> Result<bool, Error>;
}

pub struct LdeInitSystemProvider;

impl LdeInitSystemProvider {
    fn canSwitchUser(&self) -> Result<bool, Error> {
        Ok(true)
    }

    fn doSwitchUser(&self) -> Result<bool, Error> {
        let mut isinhibited = false;
        let inhibit_switchuser = 2;
        let conn = Connection::new_session()?;
        // isinhibited = dbus_call(LDE_SERVICE, LDE_PATH, LDE_INTERFACE, &conn, "IsInhibited", Some(inhibit_switchuser), Duration::from_millis(5000))?;
        
        // if isinhibited == true {
        //     isinhibited = !messageboxcheck();
        // } 
        if !isinhibited {
            let cmd = "SwitchToGreeter";
            let xdg_seat_path = env!("XDG_SEAT_PATH");
            let con = Connection::new_system()?;
            dbus_call(LIGHTDM_SERVICE, xdg_seat_path, LIGHTDM_INTERFACE, &con, cmd, None, Duration::from_millis(5000))
        } else {
            Ok(isinhibited)
        }
    }
}

impl LdePowerProvider for LdeInitSystemProvider {
    fn canAction(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerSwitchUser => return self.canSwitchUser(),
            Action::PowerReboot => "CanReboot",
            Action::PowerShutdown => "CanPowerOff",
            Action::PowerSuspend => "CanSuspend",
            Action::PowerHibernate => "CanHibernate",
            _ => return Ok(false),
        };
        println!("can cmd: {}", cmd);

        let conn = Connection::new_system()?;
        dbus_call_init_system(INIT_SYS_SERVICE, INIT_SYS_PATH, INIT_SYS_INTERFACE, &conn, cmd, false, Duration::from_millis(5000))
    }

    fn doAction(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerSwitchUser => return self.doSwitchUser(),
            Action::PowerReboot => "Reboot",
            Action::PowerShutdown => "PowerOff",
            Action::PowerSuspend => "Suspend",
            Action::PowerHibernate => "Hibernate",
            _ => return Ok(false),
        };
        println!("do cmd: {}", cmd);

        let conn = Connection::new_system()?;
        dbus_call_init_system(INIT_SYS_SERVICE, INIT_SYS_PATH, INIT_SYS_INTERFACE, &conn, cmd, true, Duration::from_millis(5000))
    }
}

pub struct LdeUIProvider;

impl LdePowerProvider for LdeUIProvider {
    fn canAction(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerLogout => "canLogout",
            Action::PowerReboot => "canReboot",
            Action::PowerShutdown => "canPowerOff",
            _ => return Ok(false),
        };
        println!("can cmd: {}", cmd);

        let mut isinhibited = false;
        let inhibit_logout = 1;
        let conn = Connection::new_session()?;
        // isinhibited = dbus_call(LDE_SERVICE, LDE_PATH, LDE_INTERFACE, &conn, "IsInhibited", Some(inhibit_logout), Duration::from_millis(5000))?;
        
        // if isinhibited == true {
        //     // isinhibited = !messageboxcheck();
        // } 
        if !isinhibited {
            dbus_call(LDE_SERVICE, LDE_PATH, LDE_INTERFACE, &conn, cmd, None, Duration::from_millis(5000))
        } else {
            Ok(isinhibited)
        }
    }

    fn doAction(&self, action: Action) -> Result<bool, Error> {
        let cmd = match action {
            Action::PowerLogout => "logout",
            Action::PowerReboot => "reboot",
            Action::PowerShutdown => "poweroff",
            _ => return Ok(false),
        };
        println!("do cmd: {}", cmd);

        let conn = Connection::new_session()?;
        dbus_call(LDE_SERVICE, LDE_PATH, LDE_INTERFACE, &conn, cmd, None, Duration::from_millis(5000))
    }
}