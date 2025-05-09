use actix::{Actor, StreamHandler, Handler, Addr, AsyncContext, ActorContext};
use actix_web::{HttpRequest, web, Responder};
use actix_web_actors::ws;
use serde::Deserialize;
use crate::database::db_setup::DbPool;
use crate::database::message::{insert_message, get_messages_for_peer};
use crate::api::hub::{Connect, Disconnect, ForwardMessage};

/// Message envoyé du Hub vers la session
#[derive(actix::Message)]
#[rtype(result = "()")] 
pub struct ServerMessage(pub String);

pub struct WsSession {
    pub peer_key: String,
    pub hub: Addr<crate::api::hub::Hub>,
    pub pool: DbPool,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Enregistrer la session dans le Hub
        self.hub.do_send(Connect {
            peer_key: self.peer_key.clone(),
            addr: ctx.address(),
        });
        // Récupérer et envoyer les messages non lus
        if let Ok(mut conn) = self.pool.get() {
            if let Ok(pending) = get_messages_for_peer(&mut conn, &self.peer_key) {
                for msg in pending {
                    ctx.text(msg.message.clone());
                }
            }
        }
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        self.hub.do_send(Disconnect {
            peer_key: self.peer_key.clone(),
        });
    }
}

#[derive(Deserialize)]
struct ClientMessage {
    to: String,
    content: String,
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("[ws] Received message: {:?}", item);
        if let Ok(ws::Message::Text(text)) = item {
            if let Ok(msg) = serde_json::from_str::<ClientMessage>(&text) {
                // Persister en base
                if let Ok(mut conn) = self.pool.get() {
                    insert_message(&mut conn, &self.peer_key, &msg.to, &msg.content, None);
                }

                // Router via le Hub
                self.hub.do_send(ForwardMessage {
                    to: msg.to.clone(),
                    payload: msg.content.clone(),
                });

                println!("Sending message to {}: {}", msg.to, msg.content);
            }
        } else if let Ok(ws::Message::Ping(p)) = item {
            ctx.pong(&p);
        } else if let Ok(ws::Message::Close(_)) = item {
            ctx.stop();
        }
    }
}

impl Handler<ServerMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// Point d’entrée WebSocket
pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<DbPool>,
    hub: web::Data<Addr<crate::api::hub::Hub>>,
) -> impl Responder {
    println!("request received: {:?}", req);
    
    // Récupérer peer_key depuis la query string
    let peer_key = req.query_string().split('&')
        .find_map(|kv| {
            let mut parts = kv.split('=');
            match (parts.next(), parts.next()) {
                (Some("peer_key"), Some(v)) => Some(v.to_string()),
                _ => None,
            }
        })
        .unwrap_or_default();

    ws::start(
        WsSession {
            peer_key,
            hub: hub.get_ref().clone(),
            pool: pool.get_ref().clone(),
        },
        &req,
        stream,
    )
}
