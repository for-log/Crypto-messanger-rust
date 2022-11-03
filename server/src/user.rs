use actix::Recipient;
use serde::{Serialize, Deserialize};
use crate::data::SystemEvent;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SafeUser {
    pub id: usize,
    pub key: String
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: usize,
    pub addr: Recipient<SystemEvent>,
    pub key: Option<String>
}

impl User {
    pub fn new(id: usize, addr: Recipient<SystemEvent>) -> Self {
        Self { id, addr, key: None }
    }
    pub fn to_safe(&self) -> SafeUser {
        SafeUser { id: self.id, key: self.key.clone().unwrap() }
    }
    pub fn is_applied(&self) -> bool {
        self.key.is_some()
    }
}