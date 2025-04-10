use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_wireguard::{
    config::{Config, Interface, Peer},
    interface::ToInterface,
    TcpStream,
    x25519::{PublicKey, StaticSecret},
};
use std::io::Error;
use base64::{engine::general_purpose::STANDARD, Engine as _};

/// Convert a base64 string to a byte array of length 32
fn bytes_from_base64(key: &str) -> [u8; 32] {
    let bytes = STANDARD.decode(key).unwrap();
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes);
    array
}

pub struct WireGuardMultiChat {
    interface: tokio_wireguard::interface::Interface,
    peer_streams: HashMap<String, TcpStream>,
}

impl WireGuardMultiChat {
    /// Set up a WireGuard interface with the given parameters
    pub async fn new(
        interface_ip: &str,
        interface_private_key: &str,
    ) -> Result<Self, Error> {
        let configuration = Config {
            interface: Interface {
                private_key: StaticSecret::from(bytes_from_base64(interface_private_key)),
                address: format!("{interface_ip}/32").parse().unwrap(),
                listen_port: None,
                mtu: Some(1280),
            },
            peers: vec![],
        };

        Ok(Self {
            interface: configuration.to_interface().await?,
            peer_streams: HashMap::new(),
        })
    }

    /// Add a peer to the WireGuard interface
    ///
    /// TODO: Use IPv6 to merge ``endpoint`` and ``peer_ip``
    pub async fn add_peer(
        &mut self,
        endpoint: &str,
        peer_ip: &str,
        public_key: &str
    ) -> Result<(), Error> {
        let peer = Peer {
            public_key: PublicKey::from(bytes_from_base64(public_key)),
            endpoint: Some(endpoint.parse().unwrap()),
            allowed_ips: vec![format!("{peer_ip}/32").parse().unwrap()],
            persistent_keepalive: Some(21),
        };
        self.interface.add_peer(peer).await?;

        let stream = TcpStream::connect(format!("{peer_ip}:49369"), &self.interface).await?;
        self.peer_streams.insert(public_key.to_string(), stream);
        Ok(())
    }

    pub async fn send_message(&mut self, public_key: &str, message: &str) -> Result<(), Error> {
        if let Some(mut peer) = self.peer_streams.get_mut(public_key) {
            peer.write_all(message.as_bytes()).await
        } else {
            Err(Error::new(std::io::ErrorKind::NotFound, "Peer not found"))
        }
    }

    pub async fn run_messaging_loop(&mut self) -> Result<(), Error> {
        let mut buffer = [0; 1024];
        loop {
            for mut peer in self.peer_streams.values_mut() {
                let n = peer.read(&mut buffer).await?;
                if n != 0 {
                    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                }
            }
        }
    }
}
