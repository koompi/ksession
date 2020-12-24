use dbus::Error;

use super::lde_power_provider::{
    LdePowerProvider, InitSystemProvider, LdeProvider
};

/// Power can perform next actions:
#[derive(Debug, Clone, Copy)]
pub enum Action {
    PowerLogout,    
    PowerHibernate, 
    PowerReboot,    
    PowerShutdown,  
    PowerSuspend,   
    PowerMonitorOff, 
    PowerShowLeaveDialog,
}

pub struct LdePower {
    providers: Vec<Box<dyn LdePowerProvider>>
}

impl Default for LdePower {
    fn default() -> Self {
        LdePower {providers: Vec::new()}
    }
}

impl LdePower {
    pub fn new() -> Self {
        Self {
            providers: vec![
                Box::new(InitSystemProvider),
                Box::new(LdeProvider)
            ]
        }
    }

    pub fn can_action(&self, action: Action) -> Result<bool, Error> {
        self.providers.iter().fold(Ok(false), |_, prod| {
            prod.can_action(action)
        })
    }

    pub fn do_action(&self, action: Action) -> Result<bool, Error> {
        self.providers.iter().fold(Ok(false), |_, prod| {
            match prod.can_action(action) {
                Ok(can_action) => match prod.do_action(action) {
                    Ok(do_action) => Ok(can_action && do_action),
                    Err(err) => Err(err)
                },
                Err(err) => Err(err)
            }
        })
    }

    pub fn can_logout(&self) -> Result<bool, Error> { self.can_action(Action::PowerLogout) }

    pub fn can_hibernate(&self) -> Result<bool, Error> { self.can_action(Action::PowerHibernate) }

    pub fn can_reboot(&self) -> Result<bool, Error> { self.can_action(Action::PowerReboot) }

    pub fn can_shutdown(&self) -> Result<bool, Error> { self.can_action(Action::PowerShutdown) }

    pub fn can_suspend(&self) -> Result<bool, Error> { self.can_action(Action::PowerSuspend) }

    pub fn can_monitor_off(&self) -> Result<bool, Error> { self.can_action(Action::PowerMonitorOff) }

    pub fn can_show_leave_dialog(&self) -> Result<bool, Error> { self.can_action(Action::PowerShowLeaveDialog) }

    pub fn logout(&self) -> Result<bool, Error> { self.do_action(Action::PowerLogout) }

    pub fn hibernate(&self) -> Result<bool, Error> { self.do_action(Action::PowerHibernate) }

    pub fn reboot(&self) -> Result<bool, Error> { self.do_action(Action::PowerReboot) }

    pub fn shutdown(&self) -> Result<bool, Error> { self.do_action(Action::PowerShutdown) }

    pub fn suspend(&self) -> Result<bool, Error> { self.do_action(Action::PowerSuspend) }

    pub fn monitor_off(&self) -> Result<bool, Error> { self.do_action(Action::PowerMonitorOff) }

    pub fn show_leave_off(&self) -> Result<bool, Error> { self.do_action(Action::PowerShowLeaveDialog) }
}