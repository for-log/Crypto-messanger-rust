pub mod rsa_crypto;
pub mod crypt;
pub mod dialogs;
pub mod dialog;
pub mod message;
pub mod wss;
pub mod data;
pub mod user;


use crypt::Crypt;
use data::{UserEvent, SystemEvent};
use futures::{stream::SplitSink, lock::Mutex, SinkExt};
use dialogs::MiniDialog;
use reqwasm::websocket::{futures::WebSocket, Message};
use serde_json::json;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use std::{sync::Arc, collections::HashMap};
use rsa_crypto::RsaCrypto;
use yew::prelude::*;
use dialog::Dialog;

enum Msg {
    SendPublicKey(String),
    Crypt(String),
    SendCrypt(String),
    SetDialog(usize),
    HandleData(String),
    AddMessage(usize, usize, String),
    AddUser(MiniDialog)
}

struct Chat {
    my_id: Option<usize>,
    rsa: Arc<Mutex<RsaCrypto>>,
    text: String,
    dialog_id: Option<usize>,
    dialogs: HashMap<usize, MiniDialog>,
    writer: Arc<Mutex<SplitSink<WebSocket, Message>>>
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let rsa = Self::get_rsa();
        let writer = wss::run("ws://127.0.0.1:8081/chat", ctx.link().callback(|x| Msg::HandleData(x)));
        Self {
            my_id: None,
            rsa,
            text: String::new(),
            dialog_id: None,
            dialogs: HashMap::new(),
            writer
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SendPublicKey(key) => {
                let data = json!(UserEvent::PublicKey(key));
                let writer_clone = self.writer.clone();
                spawn_local(async move {
                    let mut writer_lock = writer_clone.lock().await;
                    writer_lock.send(Message::Text(data.to_string())).await.unwrap();
                });
                false
            }
            Msg::Crypt(data) => {
                self.text = data;
                self.go_crypt(ctx.link().callback(|x| Msg::SendCrypt(x)));
                false
            }
            Msg::SendCrypt(s) => {
                if self.dialog_id.is_none() {
                    return false;
                }
                let mut rand_bytes = [0u8; 4];
                getrandom::getrandom(&mut rand_bytes).unwrap();
                let random_id = usize::from_be_bytes(rand_bytes);
                let message = UserEvent::Message { 
                    to: self.dialog_id.unwrap(), 
                    message: s.clone(), 
                    random_id
                };
                let data = json!(message);
                let writer_clone = self.writer.clone();
                spawn_local(async move {
                    let mut writer_lock = writer_clone.lock().await;
                    writer_lock.send(Message::Text(data.to_string())).await.unwrap();
                });
                self.dialogs.get_mut(&self.dialog_id.unwrap())
                    .and_then(|dialog| Some(dialog.add_message(random_id, self.my_id.unwrap(), self.text.clone())));
                true
            }
            Msg::SetDialog(id) => {
                self.dialogs.get_mut(&id).and_then(|user| Some({
                    user.clear();
                }));
                self.dialog_id = Some(id);
                true
            }
            Msg::AddMessage(rid, id, message) => {
                self.dialogs.get_mut(&id)
                    .and_then(|dialog| Some(
                        if let Some(cid) = self.dialog_id {
                            if cid == dialog.id {
                                dialog.add_message(rid, id, message);
                            } else {
                                dialog.add_message(rid, id, message).add_unchecked();
                            }
                        } else {
                            dialog.add_message(rid, id, message).add_unchecked();
                        }
                    ));
                true
            }
            Msg::AddUser(dialog) => {
                self.dialogs.insert(dialog.id, dialog);
                true
            }
            Msg::HandleData(data) => {
                let data = serde_json::from_str(&data).unwrap();
                let link = ctx.link();
                match data {
                    SystemEvent::YourId(id) => {self.my_id = Some(id);false}
                    SystemEvent::Message { from, message, random_id } => {
                        self.go_decrypt(message, link.callback(move |x| Msg::AddMessage(random_id, from, x)));
                        true
                    },
                    SystemEvent::SetKey(_) => todo!(),
                    SystemEvent::GetUsersIds(users) => {
                        for user in users {
                            self.parse_user(user, link.callback(|dialog| Msg::AddUser(dialog)));
                        }
                        false
                    },
                    SystemEvent::MessageStatus { random_id, status } => {false},
                    SystemEvent::UserIn(user) => {
                        self.parse_user(user, link.callback(|dialog| Msg::AddUser(dialog)));
                        false
                    },
                    SystemEvent::UserOut(user) => {
                        self.dialogs.remove(&user.id);
                        true
                    },
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let dialogs: Vec<&MiniDialog> = self.dialogs.values().collect();
        html! {
            <div class="content">
                <div class="dialogs">
                    {dialogs.into_iter().map(|x| {
                        let id = x.id;
                        let onclick = link.callback(move |_| Msg::SetDialog(id));
                        html! {
                            <div {onclick} class={format!("dialog did{}", x.id)}>
                                <div class="avatar"></div>
                                <div class="info">
                                    <p class="name">{ if x.id != self.my_id.unwrap() { format!("User#{}", x.id) } else { "Me".to_string() } }</p>
                                    if let Some(message) = &x.last_message {
                                        <p class="last-message">{ message.content.clone() }</p>
                                    }
                                </div>
                                if x.unchecked_count != 0 {
                                    <div class="checked">{x.unchecked_count}</div>
                                }
                            </div>
                        }
                    }).collect::<Html>()}
                </div>
                if let Some(dialog) = self.dialog_id {
                    <Dialog me={self.my_id.unwrap()} id={dialog} messages={self.dialogs.get(&dialog).unwrap().messages.clone()} callback={link.callback(|data: String| Msg::Crypt(data))} />
                }
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.send_public_key(ctx.link().callback(|key| Msg::SendPublicKey(key)));
        }
    }
}

fn main() {
    yew::start_app::<Chat>();
}
