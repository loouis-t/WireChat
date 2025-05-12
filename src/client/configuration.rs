use std::collections::HashMap;
use std::sync::Arc;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use tokio::sync::watch::Sender;
use onetun::config::Config;
use tokio::sync::RwLock;
use crate::wireguard::wireguard::{
    EndpointConfig,
    LocalConfig,
    start_tunnel,
};

#[derive(Deserialize, Debug)]
pub struct Endpoint {
    endpoint: String,
}

#[derive(Deserialize, Debug)]
pub struct PeerConfig {
    pub private_key: String,
    pub interface_ip: String,
    pub peer_public_key: String,
    pub peer_ip: String,
    pub endpoint: String,
}

#[derive(Clone)]
pub struct PeerConfigs {
    pub tunnels: Arc<RwLock<HashMap<String, (Sender<Config>, Config)>>>
}

impl PeerConfigs {
    pub fn new() -> Self {
        PeerConfigs {
            tunnels: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub async fn add_peer(
    data: web::Json<PeerConfig>,
    peer_configs: web::Data<PeerConfigs>,
) -> impl Responder {
    let config = LocalConfig::new(
        &data.private_key,
        &data.peer_public_key,
        &data.endpoint,
        &data.peer_ip,
        &data.interface_ip,
    );

    peer_configs.tunnels
        .write()
        .await
        .insert(data.peer_public_key.clone(), start_tunnel(Some(config)).await.unwrap());

    HttpResponse::Ok().body("WireGuard configured")
}

pub async fn update_peer_endpoint(
    data: web::Json<Endpoint>,
    peer_configs: web::Data<PeerConfigs>,
) -> impl Responder {
    if let Some(config) = peer_configs.tunnels.read().await.get(&data.endpoint) {
        let updated_config = Config::from(EndpointConfig(config.1.clone(), data.endpoint.clone()));
        if let Err(e) = config.0.send(updated_config) {
            return HttpResponse::InternalServerError().body(format!("Failed to update: {}", e));
        }

        HttpResponse::Ok().body("New WireGuard endpoint: ".to_string() + &data.endpoint)
    } else {
        HttpResponse::NotFound().body("Endpoint not found")
    }
}

