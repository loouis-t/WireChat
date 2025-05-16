use actix_web::{Responder, HttpResponse, web};
use crate::database::{db_setup::DbPool, peer::Peer};
use crate::database::peer;
use actix_web::web::Json;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MessagePayload {
    pub public_key: String,
    pub timestamp: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct GetMessagesQuery {
    pub public_key: String,
}

#[derive(Serialize)]
struct PeersResponse {
    peers: Vec<Peer>,
}

pub async fn send(payload: Json<MessagePayload>) -> impl Responder {
    HttpResponse::Ok().body(
        format!("{} - Message from {}: {}", payload.timestamp, payload.public_key, payload.message)
    )
}

pub async fn receive() -> impl Responder {
    HttpResponse::Ok().body("Receive message endpoint")
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
