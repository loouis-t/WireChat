use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ConfigData {
    private_key: String,
    peer_endpoint: String,
}

pub async fn configure_wireguard(config: web::Json<ConfigData>) -> impl Responder {
    println!("Received configuration: {:?}", config);
    HttpResponse::Ok().body("WireGuard configuration updated")
}
