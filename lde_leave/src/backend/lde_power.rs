use std::vec;

use dbus::Error;

use super::lde_power_provider::{
    LdePowerProvider, LdeInitSystemProvider, LdeUIProvider
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
    PowerSwitchUser,
}

pub struct LdePower {
    providers: Vec<Box<dyn LdePowerProvider>>
}

impl LdePower {
    pub fn new() -> Self {
        Self {
            providers: vec![
                Box::new(LdeInitSystemProvider),
                Box::new(LdeUIProvider)
            ]
        }
    }

    pub fn canAction(&self, action: Action) -> Result<bool, Error> {
        self.providers.iter().fold(Ok(false), |can_action, prod| {
            prod.canAction(action)
        })
    }

    pub fn doAction(&self, action: Action) -> Result<bool, Error> {
        self.providers.iter().fold(Ok(false), |do_action, prod| {
            match prod.canAction(action) {
                Ok(can_action) => match prod.doAction(action) {
                    Ok(do_action) => Ok(can_action && do_action),
                    Err(err) => Err(err)
                },
                Err(err) => Err(err)
            }
        })
    }
}