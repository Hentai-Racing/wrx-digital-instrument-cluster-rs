pub struct CanInterface {}

impl CanInterface {
    pub fn open(&self) -> &Self {
        #[cfg(target_os = "linux")]
        {}
        #[cfg(target_os = "windows")]
        {}
        #[cfg(target_os = "macos")]
        {}

        self
    }
}
