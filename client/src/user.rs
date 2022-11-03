use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SafeUser {
    pub id: usize,
    pub key: String
}