mod users;
mod tools;
mod game_version;
mod launcher_core;
mod json;

use users::developer_user_type::DeveloperUserType;
use tools::uuid_tools::{uuid_from_name};
use crate::{game_version::GameVersion, launcher_core::LauncherCore, tools::file_tools::lib_name_to_path};

fn main() {
    //println!("{}", test1.name());
    let test1 = DeveloperUserType::login("ZhaiShu");
    let test2 = uuid_from_name("test".to_owned());
    println!("{}", test2.as_str());
    let core: LauncherCore = LauncherCore::new("I:/Minecraft/.minecraft".to_owned());
    let test3 = GameVersion::load(core, "1.19.4".to_owned());
    println!("{}", test3.version_json.id.clone());
    //println!("{}", test3.get_arguments().join(" "));
    //println!("{}", lib_name_to_path(test3.get_libraries_and_natives().libs[0].name.clone()));
    println!("{}", test3.launch(test1))
    
}

