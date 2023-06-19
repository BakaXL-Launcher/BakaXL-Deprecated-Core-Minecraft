use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs::File;
use std::io::Read;
use crate::{json::json_version::{self, JsonVersion}, launcher_core::LauncherCore};

pub struct GameVersion {
    
}

impl GameVersion {
    pub fn load(core: LauncherCore, version: String) -> JsonVersion {
        let path = format!("{}/versions/{version}/{version}.json", core.base_path);
        let mut file = File::open(path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");
        
        let minecraft_json: JsonVersion = serde_json::from_str(&contents).expect("Failed to parse JSON");
        minecraft_json
    }
}