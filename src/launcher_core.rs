pub struct LauncherCore {
    pub base_path: String
}

impl LauncherCore {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }
}