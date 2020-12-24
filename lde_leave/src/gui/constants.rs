#[cfg(debug_assertion)]
pub const IMAGE_PATH: &'static str = "/lde_leave/assets/images";
#[cfg(not(debug_assertion))]
pub const IMAGE_PATH: &'static str = "/usr/share/lde_leave/assets/images";

