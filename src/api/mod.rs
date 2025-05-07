pub mod hub;
pub mod ws_session;

use actix_web::{web, App, HttpServer, Responder};
use actix::Actor;
use actix::Addr;
use crate::database::db_setup::DbPool;
use crate::api::hub::Hub;

/// Démarre le serveur HTTP et WebSocket
pub async fn run_api_server(pool: DbPool) -> std::io::Result<()> {
    let hub_addr: Addr<Hub> = Hub::new().start();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(hub_addr.clone()))
            .route("/", web::get().to(ws_session::ws_index))
        
    })
    .bind("0.0.0.0:49369")?
    .run()
    .await
}
