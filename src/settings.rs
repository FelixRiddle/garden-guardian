/// App wide settings
/// 
/// 
pub struct Settings {
    pub gstreamer_supported: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            gstreamer_supported: Self::is_gstreamer_supported(),
        }
    }

    fn is_gstreamer_supported() -> bool {
        gstreamer::init().is_ok()
    }
}
