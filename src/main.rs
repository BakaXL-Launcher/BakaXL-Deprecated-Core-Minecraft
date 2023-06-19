mod users;
mod tools;
mod game_version;
mod launcher_core;
mod json;

use users::developer_user_type::DeveloperUserType;
use tools::uuid_tools::{uuid_from_name};
use crate::{users::user_types::UserType, game_version::GameVersion, launcher_core::LauncherCore};

fn main() {
    let test1 = <UserType as DeveloperUserType>::new();
    println!("{}", test1.name());
    let test2 = uuid_from_name("test".to_owned());
    println!("{}", test2.as_str());
    let core: LauncherCore = LauncherCore::new("I:/Minecraft/.minecraft".to_owned());
    let test3 = GameVersion::load(core, "1.19.4".to_owned());
    println!("{}", test3.asset_index.unwrap().url)
}

