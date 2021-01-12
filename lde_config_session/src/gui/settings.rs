use config::{Config, File};
use std::path::PathBuf;

pub fn load_config(file: Option<&PathBuf>) -> Config {
    let mut s = Config::new();  
    s.merge(File::with_name(file.unwrap_or(&PathBuf::from("config/session")).to_str().unwrap())).unwrap();
    s
}