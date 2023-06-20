use std::process::Command;
use std::ptr::null;
use std::{fs::File, env::consts::OS};
use std::io::Read;
use serde_json::Value;

use crate::json::json_version::JsonAdvanceArgument;
use crate::tools::system_tools;
use crate::{json::{json_version::JsonVersion, self}, launcher_core::LauncherCore};

pub struct GameVersion {
    pub version_json: JsonVersion,
    launcher_core: LauncherCore
}

impl GameVersion {
    /// 加载版本信息
    /// 
    /// # 参数
    /// `core`: LauncherCore，需要先new一个，例如```LauncherCore::new("路径")```
    /// 
    /// `version`: String，版本名字
    /// 
    /// 加载后将返回一个GameVersion类型，然后可以获取到版本的json文件信息
    pub fn load(core: LauncherCore, version: String) -> Self {
        let path = format!("{}/versions/{version}/{version}.json", core.base_path);
        let mut file = File::open(path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");
        
        let minecraft_json: JsonVersion = serde_json::from_str(&contents).expect("Failed to parse JSON");
        Self { version_json: minecraft_json, launcher_core: core }
    }

    /// 获取加工后的启动参数，兼容以前版本的json参数`minecraftArguments`
    pub fn get_arguments(self) -> Vec<String> {
        let argument_old = self.version_json.arguments_old.unwrap_or_default();
        if !argument_old.is_empty() {
            vec![argument_old]
        } else {
            let mut argument_str: Vec<String> = vec![];
            for jvm_argument in self.version_json.arguments.clone().unwrap().jvm.unwrap() {
                match jvm_argument {
                    Value::String(s) => {
                        argument_str.push(s)
                    }
                    Value::Object(object) => {
                        let advance_argument: JsonAdvanceArgument = serde_json::from_value(serde_json::Value::Object(object)).unwrap();
                        for rule in advance_argument.rules.unwrap() {
                            if rule.action == "allow" && rule.os.is_some() {
                                if rule.os.clone().unwrap().name.is_some() {
                                    if OS.to_lowercase() != rule.os.clone().unwrap().name.unwrap_or_default().to_lowercase() {
                                        continue;
                                    } else if rule.os.clone().unwrap().version.is_some() /* 暂时还没想好怎么判断系统版本号 */ {
                                        continue;
                                    }
                                } 
                                if rule.os.clone().unwrap().arch.is_some() && rule.os.clone().unwrap().arch.unwrap() != system_tools::arch() {
                                    continue;
                                }

                                match &advance_argument.value {
                                    Value::String(s) => {
                                        argument_str.push(s.to_string())
                                    }
                                    Value::Array(arr) => {
                                        for arg in arr {
                                            argument_str.push(arg.to_string())
                                        }
                                    }
                                    _ => {
                                        
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        
                    }
                    
                }
            }

            for game_argument in self.version_json.arguments.clone().unwrap().game.unwrap() {
                match game_argument {
                    Value::String(s) => {
                        argument_str.push(s)
                    }
                    Value::Object(object) => {
                        let advance_argument: JsonAdvanceArgument = serde_json::from_value(serde_json::Value::Object(object)).unwrap();
                        for rule in advance_argument.rules.unwrap() {
                            if rule.action == "allow" {
                                match &advance_argument.value {
                                    Value::String(s) => {
                                        argument_str.push(s.to_string())
                                    }
                                    Value::Array(arr) => {
                                        for arg in arr {
                                            argument_str.push(arg.to_string())
                                        }
                                    }
                                    _ => { }
                                }
                            }
                        }
                    }
                    _ => {
                        
                    }
                    
                }
            }
            argument_str
        }
    }

    pub fn launch() {
        /*
        let command = match OS {
            "windows" => {
                Command::new("cmd")
                    .args(&["/C", ""])
            }
        };
         */
    }
}