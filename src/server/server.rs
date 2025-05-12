use actix_web::{web, Responder, HttpResponse};
use serde::Deserialize;
use crate::server::EndpointMap;

#[derive(Deserialize)]
pub struct PublicKeyJson {
    public_key: String,
}

pub async fn get_endpoint(
    data: web::Data<EndpointMap>,
    query: web::Json<PublicKeyJson>,
) -> impl Responder {
    print!("Received request for public key: {} - ", query.public_key);
    let map = data.read().unwrap();
    match map.get(&query.public_key) {
        Some(endpoint) if endpoint != "(none)" => {
            println!("Endpoint: {}", endpoint);
            HttpResponse::Ok().json(serde_json::json!({ "endpoint": endpoint }))
        }
        _ => {
            println!("Not found");
            HttpResponse::NotFound().body("Public key not found")
        }
    }
}
