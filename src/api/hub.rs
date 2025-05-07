use actix::{Actor, Addr, Context, Handler, Message};
use std::collections::HashMap;
use crate::api::ws_session::{
    ServerMessage,
    WsSession,
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub peer_key: String,
    pub addr: Addr<WsSession>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub peer_key: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ForwardMessage {
    pub to: String,
    pub payload: String,
}

pub struct Hub {
    sessions: HashMap<String, Addr<WsSession>>,
}

impl Hub {
    pub fn new() -> Self {
        Hub { sessions: HashMap::new() }
    }
}

impl Actor for Hub {
    type Context = Context<Self>;
}

impl Handler<Connect> for Hub {
    type Result = ();
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.peer_key, msg.addr);
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
            addr.do_send(ServerMessage(msg.payload));
        }
    }
}
