use actix::{Message, Recipient};
use serde::{Deserialize, Serialize};
use crate::user::SafeUser;

#[derive(Serialize, Deserialize, Message, Debug, Clone)]
#[rtype(result = "()")]
#[serde(rename_all = "snake_case")]
pub enum RawUserEvent {
    GetUsersIds { start: usize, count: usize },
    PublicKey(String),
    Message { to: usize, message: String, random_id: usize },
}

impl RawUserEvent {
    pub fn collect(&self, from_id: usize) -> UserEvent {
        match self {
            Self::GetUsersIds { start, count } => UserEvent::GetUsersIds { start: *start, count: *count, id: from_id },
            Self::PublicKey(key) => UserEvent::PublicKey { from_id, value: key.to_string() },
            Self::Message { to, message, random_id } => UserEvent::Message { from_id, to_id: *to, message: message.to_string(), random_id: *random_id },
        }
    }
}

#[derive(Serialize, Deserialize, Message, Clone, Debug)]
#[rtype(result = "()")]
#[serde(rename_all = "snake_case")]
pub enum UserEvent {
    GetUsersIds { start: usize, count: usize, id: usize },
    PublicKey { from_id: usize, value: String },
    Message { from_id: usize, to_id: usize, message: String, random_id: usize },
}

#[derive(Serialize, Deserialize, Message, Debug, Clone)]
#[rtype(result = "()")]
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

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct IDisconnect {
    pub id: usize,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct IConnect {
    pub id: usize,
    pub addr: Recipient<SystemEvent>,
}