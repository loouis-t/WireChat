use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, RwLock};
use tokio::time::{sleep, Duration};

pub mod server;

type EndpointMap = Arc<RwLock<HashMap<String, String>>>;

pub async fn server_api() -> std::io::Result<()> {
    let endpoints: EndpointMap = Arc::new(RwLock::new(HashMap::new()));
    let endpoints_clone = endpoints.clone();

    tokio::spawn(async move {
        loop {
            let new_endpoints = get_wireguard_endpoints();
            {
                let mut map = endpoints_clone.write().unwrap();
                map.clear();
                map.extend(new_endpoints);
            }
            sleep(Duration::from_secs(10)).await;
        }
    });

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(endpoints.clone()))
            .route("/get_endpoint", web::post().to(server::get_endpoint))
    })
        .bind(("0.0.0.0", 49369))?
        .run()
        .await
}

/// Query `wg show wirechat endpoints` and return a map of peer_pubkey -> endpoint
fn get_wireguard_endpoints() -> HashMap<String, String> {
    let mut endpoint_map = HashMap::new();

    let output = Command::new("wg")
        .args(&["show", "wirechat", "endpoints"])
        .output()
        .expect("Failed to execute `wg show`");

    if !output.status.success() {
        eprintln!("wg show failed: {}", String::from_utf8_lossy(&output.stderr));
        return endpoint_map;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            endpoint_map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    endpoint_map
}
