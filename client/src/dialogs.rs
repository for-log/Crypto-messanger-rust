use web_sys::CryptoKey;
use crate::message::Message;


#[derive(Clone, derive_more::From)]
pub struct MiniDialog {
    pub id: usize,
    pub last_message: Option<Message>,
    pub unchecked_count: usize,
    pub dialog_key: CryptoKey,
    pub messages: Box<Vec<Message>>,
    pub is_applied: bool
}

impl MiniDialog {
    pub fn new(id: usize, dialog_key: CryptoKey) -> Self {
        MiniDialog { 
            id, 
            last_message: None, 
            unchecked_count: 0, 
            dialog_key, 
            messages: Box::new(vec![]),
            is_applied: false
        }
    }
    pub fn clear(&mut self) -> &mut Self {
        self.unchecked_count = 0;
        self
    }
    pub fn add_unchecked(&mut self) -> &mut Self {
        self.unchecked_count += 1;
        self
    }
    pub fn add_message(&mut self, id: usize, from: usize, content: String) -> &mut Self {
        let message = Message::new(id, from, content);
        self.messages.push(message.clone());
        self.last_message = Some(message);
        self
    }
    pub fn change_applied(&mut self) {
        self.is_applied = !self.is_applied;
    }
}
