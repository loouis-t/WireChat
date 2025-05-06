use std::net::{
    Ipv4Addr,
    SocketAddr,
    SocketAddrV4,
};
use std::sync::Arc;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use onetun::config::{
    Config,
    PortForwardConfig,
    PublicKey,
    StaticSecret,
};

#[derive(Debug)]
pub struct LocalConfig {
    private_key: String,
    public_key: String,
    endpoint: String,
    peer_ip: String,
}

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
            endpoint_addr: SocketAddr::V4(
                SocketAddrV4::new(Ipv4Addr::new(83, 193, 121, 193), 51822)
            ),
            endpoint_bind_addr: SocketAddr::V4(
                SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0)
            ),
            source_peer_ip: std::net::IpAddr::V4(Ipv4Addr::new(10, 12, 17, 8)),
            keepalive_seconds: Some(21),
            max_transmission_unit: 1280,
            log: "".to_string(),
            warnings: vec![],
            pcap_file: None,
        }
    }
}

impl LocalConfig {
    pub fn new(private_key: &str, public_key: &str, endpoint: &str, peer_ip: &str) -> Self {
        LocalConfig {
            private_key: private_key.to_string(),
            public_key: public_key.to_string(),
            endpoint: endpoint.to_string(),
            peer_ip: peer_ip.to_string(),
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
