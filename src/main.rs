mod wireguard;
mod api;
mod database;

use database::{db_setup};
use dotenv::dotenv;
use std::env;
use onetun::config::Config;
use onetun::config::PortProtocol::Tcp;
use onetun::events::Bus;
use crate::wireguard::LocalConfig;

#[macro_use]
extern crate diesel;

use database::db_setup::{init, DbPool};

use tokio::net::TcpStream;
use tokio::task::LocalSet;
use tokio::io::{AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let pool = init();

    let config = LocalConfig::new(
        &env::var("iface_private_key").expect("iface_private_key not set"),
        &env::var("peer_public_key").expect("iface_public_key not set"),
        &env::var("peer_endpoint").expect("peer_endpoint not set"),
        &env::var("peer_ip").expect("peer_ip not set"),
    );

    let bus = Bus::default();
    let local = LocalSet::new();

    local.run_until(async {
        onetun::start_tunnels(Config::from(config), bus).await.unwrap();
        api::run_api_server(pool).await.unwrap();
    }).await;

    Ok(())
}
