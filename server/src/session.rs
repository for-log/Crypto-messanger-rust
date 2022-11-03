use std::time::{Duration, Instant};
use actix::{Addr, Actor, fut, Running, Handler, StreamHandler, ActorContext, AsyncContext, WrapFuture, ActorFutureExt, ContextFutureSpawner};
use actix_web_actors::ws::{WebsocketContext, ProtocolError, Message};
use serde_json::json;
use crate::{server::Server, data::{IDisconnect, IConnect, SystemEvent, RawUserEvent}};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct Session {
    pub last_ping: Instant,
    pub id: usize,
    pub public_key: Option<String>,
    pub addr: Addr<Server>
}

impl Session {
    pub fn ping(&self, ctx: &mut WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.last_ping) > CLIENT_TIMEOUT {
                act.addr.do_send(IDisconnect { id: act.id });
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
    fn handle(&mut self, msg: String) {
        let _ = serde_json::from_str(&msg.to_string())
                    .and_then(|message: RawUserEvent| 
                        Ok(self.addr.do_send(message.collect(self.id))));
    }
}

impl Actor for Session {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.ping(ctx);

        let addr = ctx.address();
        self.addr
            .send(IConnect {
                id: self.id,
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(IDisconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<SystemEvent> for Session {
    type Result = ();

    fn handle(&mut self, msg: SystemEvent, ctx: &mut Self::Context) {
        match msg {
            SystemEvent::SetKey(key) => self.public_key = Some(key),
            e => ctx.text(json!(e).to_string())
        }
    }
}

impl StreamHandler<Result<Message, ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {        
        msg.and_then(|msg| Ok(match msg {
            Message::Text(text) => {
                self.handle(text.to_string());
            },
            Message::Binary(bin) => ctx.binary(bin),
            Message::Continuation(_) => ctx.stop(),
            Message::Ping(msg) => {
                self.last_ping = Instant::now();
                ctx.pong(&msg);
            },
            Message::Pong(_) => self.last_ping = Instant::now(),
            Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            },
            Message::Nop => (),
        })).unwrap();
    }
}