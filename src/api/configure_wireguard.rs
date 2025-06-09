use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ConfigData {
    private_key: String,
    peer_endpoint: String,
}

impl Drop for ConfigData {
    fn drop(&mut self) {
        log::warn!("ConfigData est en cours de drop (!)");
    }
}

pub async fn configure_wireguard(config: web::Json<ConfigData>) -> impl Responder {
    println!("Received configuration: {:?}", config);
    HttpResponse::Ok().body("WireGuard configuration updated")
}
