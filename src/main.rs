mod client;
mod database;
mod server;
mod wireguard;

use dotenv::dotenv;
use std::env;
use onetun::config::Config;
use crate::wireguard::wireguard as wg;
extern crate diesel;
use database::db_setup::init;
use tokio::task::LocalSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();
    let local = LocalSet::new();

    // If the "--server" argument is passed, we run a tailscale-like api
    let is_server = env::args().any(|arg| arg == "--server");
    if is_server {
        local.run_until(async {
            server::server_api().await.unwrap();
        }).await;
        return Ok(());
    }
    
    let bus = onetun::events::Bus::default();
    let pool = init();

    local.run_until(async {
        onetun::start_tunnels(Config::from(wg::get_base_config()), bus.clone()).await.unwrap(); // tunnel to tailscale
        client::client_api(pool).await.unwrap();
    }).await;

    Ok(())
}
