pub mod xdg_autostart {
    pub fn desktop_files() -> Vec<std::path::PathBuf> {
        let dirs = xdg::BaseDirectories::with_prefix("autostart").unwrap();
        let listfile = dirs.list_config_files_once(dirs.get_config_dirs().get(0).unwrap());
        let mut list_file = Vec::<std::path::PathBuf>::new();
        for file in listfile {
            list_file.push(file);
        }
        list_file
    }
}
