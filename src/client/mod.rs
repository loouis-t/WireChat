pub mod hub;
pub mod ws_session;
mod configuration;
mod messenger;

use std::env;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, http};
use actix::Actor;
use actix::Addr;
use crate::database::db_setup::DbPool;
use crate::client::hub::Hub;

/// Démarre le serveur HTTP et WebSocket
pub async fn client_api(pool: DbPool) -> std::io::Result<()> {
    let hub_addr: Addr<Hub> = Hub::new().start();
    let peer_configs = configuration::PeerConfigs::new();
    let public_key = env::var("iface_public_key").expect("Missing iface_public_key in .env");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")  
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(hub_addr.clone()))
            .app_data(web::Data::new(peer_configs.clone()))
            .app_data(web::Data::new(public_key.clone()))
            .route("/", web::get().to(ws_session::ws_index))
            .route("/add_peer", web::post().to(configuration::add_peer))
            .route("/update_endpoint", web::post().to(configuration::update_peer_endpoint))
            .route("/get_peers", web::get().to(messenger::get_all_peers))
            .route("/receive", web::post().to(messenger::receive))
    })
    .bind("0.0.0.0:49369")?
    .run()
    .await
}
