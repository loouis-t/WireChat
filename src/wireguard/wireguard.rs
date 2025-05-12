use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use onetun::config::{
    Config,
    PortForwardConfig,
    PublicKey,
    StaticSecret,
};
use onetun::events::Bus;
use tokio::sync::watch::Sender;

#[derive(Debug, Clone)]
pub struct LocalConfig {
    private_key: String,
    public_key: String,
    endpoint: String,
    peer_ip: String,
    interface_ip: String,
}

pub struct EndpointConfig(pub Config, pub String);

/// Allow converting from LocalConfig to Config
/// This is useful for creating the initial config
impl From<LocalConfig> for Config {
    fn from(config: LocalConfig) -> Self {
        let (port_forwards, remote_port_forwards) =
            parse_both_port_forwards(config.peer_ip.as_str()).unwrap();
        Config {
            port_forwards,
            remote_port_forwards,
            private_key: Arc::new(StaticSecret::from(bytes_from_base64(config.private_key.as_str()))),
            endpoint_public_key: Arc::new(PublicKey::from(bytes_from_base64(config.public_key.as_str()))),
            preshared_key: None,
            endpoint_addr: SocketAddr::from_str(config.endpoint.as_str()).unwrap(),
            endpoint_bind_addr: SocketAddr::from_str("0.0.0.0:51972").unwrap(),
            source_peer_ip: IpAddr::from_str(config.interface_ip.as_str()).unwrap(),
            keepalive_seconds: Some(21),
            max_transmission_unit: 1280,
            log: "".to_string(),
            warnings: vec![],
            pcap_file: None,
        }
    }
}

/// Allow converting from EndpointConfig to Config
/// This is useful for updating the endpoint address
impl From<EndpointConfig> for Config {
    fn from(EndpointConfig(config, endpoint): EndpointConfig) -> Self {
        Config {
            endpoint_addr: SocketAddr::from_str(endpoint.as_str()).unwrap(),
            ..config
        }
    }
}

impl LocalConfig {
    pub fn new(private_key: &str, public_key: &str, endpoint: &str, peer_ip: &str, interface_ip: &str) -> Self {
        LocalConfig {
            private_key: private_key.to_string(),
            public_key: public_key.to_string(),
            endpoint: endpoint.to_string(),
            peer_ip: peer_ip.to_string(),
            interface_ip: interface_ip.to_string(),
        }
    }
}

pub fn bytes_from_base64(key: &str) -> [u8; 32] {
    let bytes = STANDARD.decode(key).unwrap();
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes);
    array
}

fn parse_both_port_forwards(ip: &str) -> Result<(Vec<PortForwardConfig>, Vec<PortForwardConfig>), Box<dyn std::error::Error>> {
    let notation = format!("127.0.0.1:8080:{}:49369", ip);

    let local = PortForwardConfig::from_notation(&notation, "127.0.0.1")?
        .into_iter()
        .collect::<Vec<_>>();

    let remote = local
        .iter()
        .cloned()
        .map(|mut pf| { pf.remote = true; pf })
        .collect::<Vec<_>>();

    Ok((local, remote))
}

/// Starts the tunnels and returns a watch::Sender to update the Config.
pub async fn start_tunnel(config: Option<LocalConfig>) -> Result<(Sender<Config>, Config), Box<dyn std::error::Error>> {
    let mut local_config = get_base_config();
    if let Some(cfg) = config {
        local_config = cfg;
    }

    let (cfg_tx, mut cfg_rx) = watch::channel(Config::from(local_config.clone()));
    let mut current_handle: Option<tokio::task::JoinHandle<()>> = None;

    tokio::spawn(async move {
        while cfg_rx.changed().await.is_ok() {
            let new_cfg = cfg_rx.borrow().clone();
            println!("New config received, restarting tunnels: {}", new_cfg.endpoint_addr);

            // If there's a running tunnel, abort it
            if let Some(handle) = current_handle.take() {
                handle.abort(); // forcefully cancel the task
                let _ = handle.await; // wait for it to shut down
            }

            // Spawn a fresh tunnel task
            let cfg_clone = new_cfg.clone();
            let bus = Bus::default();
            let handle = tokio::spawn(async move {
                if let Err(e) = onetun::start_tunnels(cfg_clone, bus).await {
                    eprintln!("tunnels exited: {:?}", e);
                }
            });
            current_handle = Some(handle);
        }
    });

    Ok((cfg_tx, Config::from(local_config)))
}

pub fn get_base_config() -> LocalConfig {
    LocalConfig::new(
        &env::var("iface_private_key").expect("Missing iface_private_key in .env"),
        &env::var("peer_public_key").expect("Missing peer_public_key in .env"),
        &env::var("peer_endpoint").expect("Missing peer_endpoint in .env"),
        &env::var("peer_ip").expect("Missing peer_ip in .env"),
        &env::var("iface_ip").expect("Missing iface_ip in .env"),
    )
}
