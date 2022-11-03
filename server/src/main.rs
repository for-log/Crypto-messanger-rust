pub mod data;
pub mod server;
pub mod session;
pub mod user;

use std::{time::Instant, sync::{Arc, Mutex}};
use actix::{Addr, Actor};
use actix_web::{web::{self, Data}, App, Error, HttpRequest, HttpResponse, HttpServer, get};
use actix_web_actors::ws;
use serde_json::json;
use server::Server;

use crate::{session::Session, data::RawUserEvent};


#[get("/chat")]
async fn index(req: HttpRequest, stream: web::Payload, srv: Data<Addr<Server>>, counter: Data<Arc<Mutex<usize>>>) -> Result<HttpResponse, Error> {
    let mut counter = counter.lock().unwrap();
    *counter += 1;
    ws::start(
        Session {
            id: *counter-1, 
            last_ping: Instant::now(), 
            public_key: None,
            addr: srv.get_ref().clone()
        }, 
        &req, 
        stream)
}

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    println!("{}", json!(RawUserEvent::PublicKey("HELLO WORLD".to_string())));

    let counter = Arc::new(Mutex::new(0usize));
    let manager = Server::new().start();
    HttpServer::new(move || 
        App::new()
            .service(index)
            .app_data(Data::new(manager.clone()))
            .app_data(Data::new(counter.clone()))
    )
    .bind("127.0.0.1:8081").unwrap()
    .run()
    .await.unwrap();
}