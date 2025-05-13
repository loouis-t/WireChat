use actix::{Actor, Addr, Context, Handler, Message};
use std::collections::HashMap;
use crate::client::messenger::MessagePayload;
use crate::client::ws_session::{
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

impl Handler<MessagePayload> for Hub {
    type Result = ();
    fn handle(&mut self, msg: MessagePayload, _: &mut Context<Self>) {
        if let Some(addr) = self.sessions.get(&msg.to) {
            addr.do_send(ServerMessage(msg.message));
        }
    }
}
