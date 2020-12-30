pub fn run_client() {
    println!("run dbus from client .... ");
}
use dbus::blocking::Connection;
use std::time::Duration;
pub enum Variant {}
pub fn run_dbus(
    service: &str,
    obj_path: &str,
    method: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;

    let proxy = conn.with_proxy(service, obj_path, Duration::from_millis(100));

    let (name,): (Vec<String>,) = proxy.method_call(service, method, ())?;
    Ok(name)
}

pub fn dbus_call(
    service: &str,
    obj_path: &str,
    method: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy(service, obj_path, Duration::from_millis(100));

    let (status,): (bool,) = proxy.method_call(service, method, ())?;
    Ok(status)
}
