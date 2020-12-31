pub mod xdg_autostart {
    pub fn desktop_files() -> Vec<String> {
        let dirs = xdg::BaseDirectories::with_prefix("autostart").unwrap();
        let mut listfile = dirs.list_config_files(dirs.get_config_dirs().get(0).unwrap());
        let mut list_file = Vec::<String>::new();
        listfile.iter_mut().for_each(|file| {
            list_file.push(file.file_name().unwrap().to_str().unwrap().to_string());
        });
        list_file
    }
}
