use actix::{Addr, Message};
use actix_web::{Responder, HttpResponse, web};
use crate::database::{db_setup::DbPool, peer::Peer};
use crate::database::peer;
use actix_web::web::Json;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Message, Debug)]
#[rtype(result = "()")]
pub struct MessagePayload {
    pub from: String,
    pub to: String,
    pub timestamp: String,
    pub message: String,
}

#[derive(Serialize)]
struct PeersResponse {
    peers: Vec<Peer>,
}

pub async fn send(payload: MessagePayload) {
    reqwest::Client::new()
        .post("http://127.0.0.1:8080/receive")
        .body(payload.message.clone())
        .send().await.unwrap();
    
    println!(
        "{} - Message sent from {} to {}: {}",
        payload.timestamp, payload.from, payload.to, payload.message
    );
}

pub async fn receive(
    public_key: web::Data<String>,
    payload: Json<MessagePayload>,
    hub: web::Data<Addr<crate::client::hub::Hub>>,
) -> impl Responder {
    println!("[receive] Received message: {:?}", payload);
    if payload.to.as_str() != public_key.get_ref().as_str() {
        eprintln!("[receive] Message not for you: {}", payload.to);
        return HttpResponse::BadRequest().finish();
    }
    println!("{} - Message received from {}: {}", payload.timestamp, payload.from, payload.message);
    hub.do_send(MessagePayload {
        from: payload.from.clone(),
        to: payload.to.clone(),
        timestamp: payload.timestamp.clone(),
        message: payload.message.clone(),
    });

    HttpResponse::Ok().body("Message received")
}

pub async fn get_all_peers(
    pool: web::Data<DbPool>,
) -> impl Responder {
    // 1) exécution dans un thread‐pool
    let query = web::block(move || {
        let mut conn = pool.get()
            .map_err(|e| diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            ))?;
        peer::get_all_peers(&mut conn)
    })
    .await;

    // 2) on déplie le résultat
    let peers_vec = match query {
        Ok(Ok(vec)) => vec,
        Ok(Err(e)) => {
            eprintln!("[get_all_peers] DB error: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
        Err(e) => {
            eprintln!("[get_all_peers] DB error: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // 3) on renvoie l’objet { peers: [...] }
    HttpResponse::Ok().json(PeersResponse { peers: peers_vec })
}
