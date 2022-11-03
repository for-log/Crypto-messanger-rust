use std::sync::Arc;
use futures::{StreamExt, stream::SplitSink, lock::Mutex};
use reqwasm::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;
use yew::Callback;


pub fn run(addr: &str, callback: Callback<String>) -> Arc<Mutex<SplitSink<WebSocket, Message>>> {
    WebSocket::open(addr)
    .and_then(|socket| {
        let (writer, mut reader) = socket.split();
        spawn_local({
            async move {
                while let Some(msg) = reader.next().await {
                    match msg {
                        Ok(Message::Text(data)) => {
                            callback.emit(data);
                        }
                        Ok(Message::Bytes(_)) => {
                            break;
                        }
                        Err(e) => {
                            panic!("ws: {:?}", e)
                        }
                    }
                }
            }
        });
        Ok(Arc::new(Mutex::new(writer)))
    }).unwrap()
}