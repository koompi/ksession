use dbus::{
    blocking::Connection, Error
};
use std::time::Duration;

pub fn print_dbus_msg(err: Error) {
    eprintln!("======= D-Bus error =======");
    eprintln!("Name: {}", err.name().unwrap());
    eprintln!("Message: {}", err.message().unwrap());
}

pub(super) fn dbus_call(service: &str, path: &str, interface: &str, conn: &Connection, method: &str, args: Option<u8>, dur: Duration) -> Result<bool, Error> {
    let proxy = conn.with_proxy(service, path, dur);
    let (result,) = match args {
        Some(arg) => proxy.method_call(interface, method, (arg, ))?,
        None => proxy.method_call(interface, method, ())?
    };
    
    Ok(result)
}

pub(super) fn dbus_call_init_system(service: &str, path: &str, interface: &str, conn: &Connection, method: &str, need_arg: bool, dur: Duration) -> Result<bool, Error> {
    let proxy = conn.with_proxy(service, path, dur);
    let result: Result<(String,), Error> = proxy.method_call(interface, method, (need_arg,));
    match result {
        Ok((res,)) => {
            println!("systemd: {} = {}", method, res);
            Ok(res == "yes")
        },
        Err(err) => Err(err)
    }
}