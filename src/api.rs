use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ConfigData {
    private_key: String,
    peer_endpoint: String,
    // ... add other configuration fields as needed
}

async fn configure_wireguard(config: web::Json<ConfigData>) -> impl Responder {
    // Here, call your wireguard_mod function to update configuration.
    // For example, you might update global state or trigger a re‑configuration.
    println!("Received configuration: {:?}", config);

    // For demo, we simply respond "OK"
    HttpResponse::Ok().body("WireGuard configuration updated")
}

pub async fn run_api_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/configure", web::post().to(configure_wireguard))
        // add more routes for status, messaging, etc.
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
