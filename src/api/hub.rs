use actix::{Actor, Addr, Context, Handler, Message};
use std::collections::HashMap;
use crate::api::ws_session::ServerMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub peer_key: String,
    pub addr: Addr<crate::api::ws_session::WsSession>,
}

impl Drop for Connect {
    fn drop(&mut self) {
        log::warn!("Connect est en cours de drop (!)");
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub peer_key: String,
}

impl Drop for Disconnect {
    fn drop(&mut self) {
        log::warn!("Disconnect est en cours de drop (!)");
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ForwardMessage {
    pub to: String,
    pub payload: String,
}

impl Drop for ForwardMessage {
    fn drop(&mut self) {
        log::warn!("ForwardMessage est en cours de drop (!)");
    }
}

pub struct Hub {
    sessions: HashMap<String, Addr<crate::api::ws_session::WsSession>>,
}

impl Hub {
    pub fn new() -> Self {
        Hub { sessions: HashMap::new() }
    }
}

impl Actor for Hub {
    type Context = Context<Self>;
}

impl Drop for Hub {
    fn drop(&mut self) {
        log::warn!("Hub est en cours de drop (!)");
    }
}

impl Handler<Connect> for Hub {
    type Result = ();
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.peer_key.clone(), msg.addr.clone());
    }
}

impl Handler<Disconnect> for Hub {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.peer_key);
    }
}

impl Handler<ForwardMessage> for Hub {
    type Result = ();
    fn handle(&mut self, msg: ForwardMessage, _: &mut Context<Self>) {
        if let Some(addr) = self.sessions.get(&msg.to) {
            addr.do_send(ServerMessage(msg.payload.clone()));
        }
    }
}
