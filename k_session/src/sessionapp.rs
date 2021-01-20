pub trait SessionApplication {
    fn set_windowmanager();
    fn set_config();
    fn startup();

    fn load_enviromentsettings();
    fn load_keyboardsettings();
    fn load_mousesettings();
}
