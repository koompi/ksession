use is_executable::IsExecutable;
use std::env::var_os;
use std::path::Path;
pub struct WinowManager {
    name: String,
    command: String,
    commment: String,
    exists: bool,
}
pub type WinowManagerList = Vec<WinowManager>;

pub fn getWMList(onlyAvailable: Option<bool>) -> Option<(WinowManagerList, ())> {
    None
}
pub fn find_program(name: &str) -> bool {
    let abs_path = format!("/usr/bin/{}", name);
    let path = Path::new(&abs_path);
    if path.is_executable() {
        true
    } else {
        if let Some(val) = var_os("PATH") {
            let paths = val.to_str().unwrap().split(':');
            for p in paths {
                let file = String::from(format!("{}/{}", p, name));
                if Path::new(&file).is_executable() {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }
}
