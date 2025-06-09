pub mod hub;
pub mod ws_session;

use actix::Actor;
use actix::Addr;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};

use crate::database::db_setup::DbPool;
use crate::api::hub::Hub;


async fn health_check() -> impl Responder {
    log::info!("Health check received");
    HttpResponse::Ok().body("Healthy")
}

/// Démarre le serveur HTTP et WebSocket
pub async fn run_api_server(pool: DbPool) -> std::io::Result<()> {
    let hub_addr: Addr<Hub> = Hub::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(hub_addr.clone()))
            // Route de test
            .route("/health", web::get().to(health_check))
            // Route pour l'établissement du WebSocket
            .route("/ws/", web::get().to(ws_session::ws_index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
