use std::fmt::format;

pub struct LauncherCore {
    pub base_path: String,
    pub assets_path: String,
}

impl LauncherCore {
    /// 初始化启动核心的路径
    pub fn new(path: String) -> Self {
        Self { base_path: path.clone(), assets_path: format!("{}", path) }
    }
    
    /// 手动设置资源文件（包括libraries）的路径
    /// 
    /// 该方法可以让所有mc游戏共用同一个资源文件
    pub fn set_assets_path(&mut self, path: String) {
        self.assets_path = path;
    }
}