use base64::{engine::general_purpose::STANDARD, Engine as _};
use onetun::config::{Config, PortForwardConfig, PublicKey, StaticSecret};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, IpAddr};
use std::sync::Arc;


#[derive(Debug)]
pub struct LocalConfig {
    endpoint: String,
    peer_ip: String,
    private_key: String,
    public_key: String,
    source_peer_ip: IpAddr,
}

impl Drop for LocalConfig {
    fn drop(&mut self) {
        log::warn!("LocalConfig est en cours de drop (!)");
    }
}

impl From<LocalConfig> for Config {
    fn from(config: LocalConfig) -> Self {
        let (port_forwards, remote_port_forwards) =
            parse_both_port_forwards(config.peer_ip.as_str()).unwrap();

        let endpoint_addr = match config.endpoint.parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(e) => {
                eprintln!("'peer_endpoint' invalide : '{}'", config.endpoint);
                eprintln!("Detail de l’erreur : {}", e);
                std::process::exit(1);
            }
        };

        Config {
            endpoint_addr,
            endpoint_bind_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)),
            endpoint_public_key: Arc::new(PublicKey::from(bytes_from_base64(config.public_key.as_str()))),
            keepalive_seconds: Some(21),
            log: "".to_string(),
            max_transmission_unit: 1280,
            pcap_file: None,
            port_forwards,
            preshared_key: None,
            private_key: Arc::new(StaticSecret::from(bytes_from_base64(config.private_key.as_str()))),
            remote_port_forwards,
            source_peer_ip: config.source_peer_ip,
            warnings: vec![],
        }
    }
}

impl LocalConfig {
    pub fn new(private_key: &str, public_key: &str, endpoint: &str, peer_ip: &str) -> Self {
        let source_peer_ip = std::env::var("SOURCE_PEER_IP")
            .ok()
            .and_then(|ip| ip.parse::<IpAddr>().ok())
            .unwrap_or(IpAddr::V4(Ipv4Addr::new(10, 12, 17, 8)));

        LocalConfig {
            endpoint: endpoint.to_string(),
            peer_ip: peer_ip.to_string(),
            private_key: private_key.to_string(),
            public_key: public_key.to_string(),
            source_peer_ip,
        }
    }
}

pub fn bytes_from_base64(key: &str) -> [u8; 32] {
    let bytes = STANDARD.decode(key)
        .expect("Cle en base64 invalide : decodage impossible");
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
