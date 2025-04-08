mod wireguard_mod;
mod api;
mod messaging;
mod database;
use database::{db_setup, peer};

#[macro_use]
extern crate diesel;

// This module would handle sending/receiving messages over the WireGuard tunnel
use std::io;

use tokio::join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    db_setup::init();

    let peer_ip = "10.12.17.1";

    // Connect to Yan
    let interface = wireguard_mod::setup_wireguard_interface(
        "10.12.17.8",
        "ip:port",
        peer_ip,
        "",
        "",
    ).await?;

    // wireguard_mod::send_message_peer(peer_ip, b"Hello from Louis", &interface).await?;

    // Step 2: Run the API server concurrently.
    // Optionally, you can also spawn your messaging tasks here.
    // let api_future = api::run_api_server();
    // If you have a messaging loop, include it as another future.
    // let messaging_future = messaging::run_messaging_loop();

    // Run both concurrently. If you don’t need messaging separately, just run the API.
    // join!(
    //     api_future,
    //     // messaging_future
    // );
    Ok(())
}
