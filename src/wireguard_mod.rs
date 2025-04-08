use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_wireguard::{
    config::{Config, Interface, Peer},
    interface::ToInterface,
    TcpStream,
    x25519::{PublicKey, StaticSecret},
};
use std::io::Error;
use base64::decode;

/// Convert a base64 string to a byte array of length 32
fn bytes_from_base64(key: &str) -> [u8; 32] {
    let bytes = decode(key).unwrap();
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes);
    array
}

/// Set up a WireGuard interface with the given parameters
pub async fn setup_wireguard_interface(
    interface_ip: &str,
    peer_endpoint: &str,
    allowed_ip: &str,
    interface_private_key: &str,
    peer_public_key: &str,
) -> Result<(), Error> {

    // println!("[*] Setting up WireGuard interface with IP: {interface_ip}, endpoint: {peer_endpoint}, allowed IP: {allowed_ip}");

    let bytes = decode("").unwrap();
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes);
    let public_cle = PublicKey::from(array);

    let bytes = decode("").unwrap();
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes);
    let private_cle = StaticSecret::from(array);

    let configuration = Config {
        interface: Interface {
            private_key: private_cle,
            address: "10.0.0.6/32".parse().unwrap(),
            listen_port: None,
            mtu: Some(1280),
        },
        peers: vec![Peer {
            public_key: public_cle,
            endpoint: Some("ip:port".parse().unwrap()),
            allowed_ips: vec!["192.168.0.70/32".parse().unwrap()],
            persistent_keepalive: Some(21),
        }],
    };

    let interface = configuration.to_interface().await?;
    println!("interface is closed ? : {:?}", interface.is_closed());

    let mut stream = TcpStream::connect("192.168.0.70:8081", &interface).await?;
    // println!("[*] Sending stream: {stream:?}");
    stream.write_all(b"Bonjour").await?;

    // read from the stream
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    // convert from bytes to string
    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Received: {}", response);

    // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    interface.close();

    Ok(())
}

// /// Send a message to a peer over the WireGuard tunnel
// pub async fn send_message_peer(ip: &str, message: &[u8], interface: &Interface) -> Result<(), Error> {
//     println!("[*] WireGuard configuration: {interface:?}");
//     println!("[*] Sending message to peer: {ip}");
//     let mut stream = TcpStream::connect(format!("{ip}:8080").as_str(), interface).await?;
//     println!("[*] Sending stream: {stream:?}");
//     stream.write_all(message).await
// }
