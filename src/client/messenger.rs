use actix_web::{Responder, HttpResponse, Handler};
use actix_web::web::Json;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct MessagePayload {
    pub public_key: String,
    pub timestamp: String,
    pub message: String,
}

pub async fn send(payload: Json<MessagePayload>) -> impl Responder {
    HttpResponse::Ok().body(
        format!("{} - Message from {}: {}", payload.timestamp, payload.public_key, payload.message)
    )
}

pub async fn receive() -> impl Responder {
    HttpResponse::Ok().body("Receive message endpoint")
}
