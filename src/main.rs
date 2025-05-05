mod wireguard_mod;
mod api;
mod database;

use database::{db_setup, peer};
use wireguard_mod::WireGuardMultiChat;
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;

// This module would handle sending/receiving messages over the WireGuard tunnel
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let mut db = db_setup::init();

    // Set up the WireGuard interface
    let mut chat = WireGuardMultiChat::new(
        env::var("iface_ip").expect("iface_ip not set").as_str(),
        env::var("iface_private_key").expect("iface_private_key not set").as_str(),
    ).await?;

    let peer_public_key = env::var("peer_public_key").expect("peer_public_key not set");

    // Add a peer to the WireGuard interface
    chat.add_peer(
        env::var("peer_endpoint").expect("peer_endpoint not set").as_str(),
        env::var("peer_ip").expect("peer_ip not set").as_str(),
        peer_public_key.as_str(),
    ).await?;

    // Contact the peer
    chat.send_message(peer_public_key.as_str(), "Hello, peer!\n").await?;

    // // Step 2: Run the API server concurrently.
    // // Optionally, you can also spawn your messaging tasks here.
    // // let api_future = api::run_api_server();
    // If you have a messaging loop, include it as another future.
    let messaging_future = chat.run_messaging_loop();

    // Run both concurrently. If you don’t need messaging separately, just run the API.
    join!(
        // api_future,
        messaging_future,
    );
    Ok(())
}
