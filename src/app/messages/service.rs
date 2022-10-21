use crate::app::User;

use super::data::IndexData;

pub fn index() -> IndexData {
    IndexData {
        messages: String::from("MESSAGES"),
    }
}

pub fn create(user: User) -> IndexData {
    dbg!(user);

    IndexData {
        messages: String::from("MESSAGES"),
    }
}
