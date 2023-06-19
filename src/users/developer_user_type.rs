use crate::tools::uuid_tools::uuid_from_name;

use super::user_types::{UserType, UserResult};

pub trait DeveloperUserType {
    fn new() -> Self;
    fn login(username: &str) -> UserResult;
}

impl DeveloperUserType for UserType {
    fn new() -> Self {
        Self { type_name: "Developer".to_owned() }
    }

    fn login(mut username: &str) -> UserResult {
        <UserResult as DeveloperUserResult>::login(&mut username)
    }
}

pub trait DeveloperUserResult {
    fn login(username: &str) -> Self;
}


impl DeveloperUserResult for UserResult {
    fn login(username: &str) -> UserResult {
        Self { username: username.to_owned(), uuid: uuid_from_name(username.to_owned()) }
    }

}