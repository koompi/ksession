use wmctrl;

pub struct ModuleManager {
    wmStarted: bool,
    mTrayStarted: bool,
}

pub fn get_wm() -> String {
    let st = String::from_utf8(wmctrl::show_wm_information().stdout).unwrap();

    let l1 = st
        .split('\n')
        .map(|f| f.to_string())
        .collect::<Vec<String>>()[0]
        .clone()
        .split(':')
        .map(|f| f.to_string())
        .collect::<Vec<String>>()[1]
        .clone()
        .chars()
        .into_iter()
        .filter(|s| !s.is_whitespace())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("");
    l1
}
