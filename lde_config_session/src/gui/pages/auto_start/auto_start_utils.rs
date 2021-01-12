use std::path::PathBuf;
use freedesktop_entry_parser::parse_entry;

pub struct AutostartUtils;

impl AutostartUtils {
    pub fn show_only_in_lde(file: PathBuf) -> bool {
        let entry = parse_entry(file).unwrap();
        if let Some(only_show_in) = entry.section("Desktop Entry").attr("OnlyShowIn") {
            only_show_in.contains("LDE;")
        } else {
            false
        }
    } 

    pub fn is_lde_module(file: PathBuf) -> bool {
        let entry = parse_entry(file).unwrap();
        if let Some(is_lde_mod) = entry.section("Desktop Entry").attr("X-LDE-Module") {
            is_lde_mod.parse().unwrap()
        } else {
            false
        }
    }
}