use crate::tools::uuid_tools::uuid_from_name;

use super::user_types::{UserResult};

pub struct DeveloperUserType {
    
}

impl DeveloperUserType {
    pub fn login(mut username: &str) -> UserResult {
        UserResult::Developer { username: username.to_owned(), uuid: uuid_from_name(username.to_owned()) }
    }
}