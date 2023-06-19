
pub struct UserType {
    pub type_name: String
}

pub struct UserResult {
    pub username: String,
    pub uuid: String
}

impl UserType {
    pub fn name(self) -> String {
        self.type_name
    }
}

impl UserResult {}