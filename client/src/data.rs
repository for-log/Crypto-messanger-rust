use serde::{Deserialize, Serialize};

use crate::user::SafeUser;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UserEvent {
    GetUsersIds { start: usize, count: usize },
    PublicKey(String),
    Message { to: usize, message: String, random_id: usize },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SystemEvent {
    YourId(usize),
    Message { from: usize, message: String, random_id: usize },
    SetKey(String),
    GetUsersIds(Vec<SafeUser>),
    MessageStatus { random_id: usize, status: bool },
    UserIn(SafeUser),
    UserOut(SafeUser),
}