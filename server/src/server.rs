use std::collections::HashMap;
use actix::{Context, Actor, Handler};
use crate::{
    data::{IConnect, IDisconnect, UserEvent, SystemEvent}, user::User, 
};

#[derive(Clone)]
pub struct Server {
    pub sessions: HashMap<usize, User>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            sessions: HashMap::new(),
        }
    }
    fn send_message(&self, to: usize, message: SystemEvent) -> bool {
        self.sessions.get(&to)
            .and_then(|user| 
                Some(user.addr.do_send(message))
            ).is_some()
    }
    fn set_key(&mut self, id: usize, pkey: String) {
        self.sessions.get_mut(&id)
            .and_then(|user| {
                user.key = Some(pkey.clone());
                Some(user.addr.do_send(SystemEvent::SetKey(pkey)))
            });
    }
    fn get_users(&self, id: usize, start: usize, count: usize) {
        self.sessions.get(&id)
            .and_then(|user| 
                Some(user.addr.do_send(SystemEvent::GetUsersIds(
                    self.sessions.values()
                        .skip(start).take(count).filter(|x| x.is_applied()).map(|x| x.to_safe()).collect()
                )))
            );
    }
    fn send_all(&self, data: SystemEvent, addr: Option<&User>) {
        for user in self.sessions.values() {
            if user.is_applied() {
                user.addr.do_send(data.clone());
                addr.and_then(|other| 
                    Some(if other.id != user.id {other.addr.do_send(SystemEvent::UserIn(user.to_safe()))}));
            }
        }
    }
}


impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<IConnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: IConnect, _: &mut Context<Self>) -> Self::Result {
        println!("[+] New connection [+]");
        msg.addr.do_send(SystemEvent::YourId(msg.id));
        self.sessions.insert(msg.id, User::new(msg.id, msg.addr));
    }
}

impl Handler<IDisconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: IDisconnect, _: &mut Context<Self>) {
        println!("[+] Disconnect [+]");
        let user = self.sessions.remove(&msg.id).unwrap();
        self.send_all(SystemEvent::UserOut(user.to_safe()), None);
    }
}

impl Handler<UserEvent> for Server {
    type Result = ();

    fn handle(&mut self, msg: UserEvent, _: &mut Context<Self>) {
        println!("[+] Data: {:?} [+]", msg);
        match msg {
            UserEvent::GetUsersIds { id, start, count } => self.get_users(id, start, count),
            UserEvent::PublicKey { from_id, value } => {
                self.set_key(from_id, value.clone());
                self.sessions.get(&from_id).and_then(|user| 
                    Some(self.send_all(SystemEvent::UserIn(user.to_safe()), Some(&user))));
            },
            UserEvent::Message { message, from_id, to_id, random_id } => {self.send_message(
                from_id, 
                SystemEvent::MessageStatus { random_id, status: self.send_message(
                    to_id, 
                    SystemEvent::Message { from: from_id, message, random_id }
                )}
            );}
        };
    }
}

