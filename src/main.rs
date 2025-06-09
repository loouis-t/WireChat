mod api;
mod database;
mod wireguard;

struct DropSentinel;

use crate::database::db_setup::init;

use anyhow::{anyhow, Result};

use base64::engine::general_purpose;
use base64::Engine;

use onetun::config::Config;
use onetun::events::Bus;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::path::Path;
use std::sync::Arc;

use tokio::runtime::Builder;
use tokio::task::LocalSet;

use x25519_dalek::{PublicKey, StaticSecret};


// Wrapper pour chaque ressource à tracer
struct Traced<T> {
    inner: T,
    _drop: DropSentinel,
}

impl<T> Traced<T> {
    fn new(inner: T) -> Self {
        Traced {
            inner,
            _drop: DropSentinel,
        }
    }
}

// Traçage des zones désallouées
impl Drop for DropSentinel {
    fn drop(&mut self) {
        log::warn!("DropSentinel a été droppé (!)");
    }
}

// Convertit une clé privée base64 en StaticSecret
fn decode_private_key(key: &str) -> Result<Arc<StaticSecret>> {
    let bytes = general_purpose::STANDARD.decode(key.trim())?;
    let array: [u8; 32] = bytes.try_into().map_err(|_| anyhow!("Clé privée invalide"))?;
    Ok(Arc::new(StaticSecret::from(array)))
}

// Convertit une clé publique base64 en PublicKey
fn decode_public_key(key: &str) -> Result<Arc<PublicKey>> {
    let bytes = general_purpose::STANDARD.decode(key.trim())?;
    let array: [u8; 32] = bytes.try_into().map_err(|_| anyhow!("Clé publique invalide"))?;
    Ok(Arc::new(PublicKey::from(array)))
}

// Convertit un preshared key en tableau de 32 octets
fn decode_preshared_key(key: &str) -> Result<[u8; 32]> {
    let bytes = general_purpose::STANDARD.decode(key.trim())?;
    if bytes.len() != 32 {
        return Err(anyhow!("PresharedKey invalide : doit faire 32 octets"));
    }
    Ok(bytes.try_into().map_err(|_| anyhow!("PresharedKey : conversion échouée"))?)
}

// Résolution DNS manuelle avant parsing
fn resolve_to_socketaddr(s: &str) -> Result<SocketAddr> {
    s.to_socket_addrs()?
        .next()
        .ok_or_else(|| anyhow!("Échec de la résolution DNS"))
}

// Parse un fichier WireGuard `.conf` vers un `onetun::Config`
fn parse_wireguard_to_onetun_config(conf: &str) -> Result<Config> {
    let mut private_key = None;
    let mut public_key = None;
    let mut preshared_key = None;
    let mut allowed_ips = Vec::new();
    //let mut endpoint = None;
    let mut endpoint = None::<String>;
    let mut source_ip = None;
    let mut current_section = String::new();

    for line in conf.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len() - 1].to_string();
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();

        match (current_section.as_str(), key) {
            ("Interface", "PrivateKey") => private_key = Some(decode_private_key(value)?),
            //("Interface", "Address") => source_ip = Some(value.parse::<IpAddr>()?),
            ("Interface", "Address") => {
                let ip_part = value.split('/').next().ok_or_else(|| anyhow!("Adresse invalide"))?;
                source_ip = Some(ip_part.parse::<IpAddr>()?);
            }
            ("Peer", "PublicKey") => public_key = Some(decode_public_key(value)?),
            ("Peer", "PresharedKey") => preshared_key = Some(decode_preshared_key(value)?),
            ("Peer", "AllowedIPs") => allowed_ips.push(value.to_string()),
            //("Peer", "Endpoint") => endpoint = Some(value.parse::<SocketAddr>()?),
            ("Peer", "Endpoint") => endpoint = Some(value.to_string()),
            _ => {}
        }
    }

    Ok(Config {
        private_key: private_key.ok_or_else(|| anyhow!("PrivateKey manquant"))?,
        endpoint_public_key: public_key.ok_or_else(|| anyhow!("PublicKey manquant"))?,
        preshared_key,
        //endpoint_addr: endpoint.ok_or_else(|| anyhow!("Endpoint manquant"))?,
        endpoint_addr: resolve_to_socketaddr(&endpoint.ok_or_else(|| anyhow!("Endpoint manquant"))?)?,
        source_peer_ip: source_ip.ok_or_else(|| anyhow!("Address manquant"))?,
        keepalive_seconds: Some(25),
        endpoint_bind_addr: "0.0.0.0:0".parse::<SocketAddr>()?,
        max_transmission_unit: 1420, // Valeur classique WireGuard
        port_forwards: vec![],
        remote_port_forwards: vec![],
        log: String::new(),
        warnings: vec![],
        pcap_file: None,
    })
}

fn ensure_db_exists(db_path: &Path) -> Result<(), std::io::Error> {
    if !db_path.exists() {
        File::create(db_path)?; // Crée un fichier vide si jamais la base de données n'a pas encore été créée
    }
    Ok(())
}

fn launch_backend() -> Result<(), Box<dyn Error>> {
    /*
    dotenv().ok();
    log::debug!("Fichier '.env'  chargé");
    */

    const SQLITE_FILE_NAME: &str = "db.sqlite";

    let config_path = Path::new("config/pacome-wirechat.conf");

    /*
    if !config_path.exists() {
        eprintln!(
            "Fichier de configuration WireGuard introuvable : {:?}",
            config_path
        );
        std::process::exit(1);
    }
    */
    if !config_path.exists() {
        return Err("Fichier de configuration introuvable".into());
    }

    log::debug!("Lecture du fichier de configuration");
    let config_content = fs::read_to_string(config_path).expect("Échec de la lecture du fichier WireGuard");
    log::debug!("Parsing de la configuration WireGuard");
    let config = parse_wireguard_to_onetun_config(&config_content).expect("Erreur lors du parsing de la configuration WireGuard");

    log::debug!("Initialisation du bus d'événements");
    let bus = Traced::new(Bus::default());

    /*
    let db_path = config_path.with_file_name(SQLITE_FILE_NAME);
    log::debug!("Chemin de la base de données : {:?}", db_path);
    */
    let db_path = config_path.with_file_name(SQLITE_FILE_NAME);
    log::debug!("Chemin de la base de données : {:?}", db_path);

    /*
    let pool = Traced::new(init(db_path));
    */
    ensure_db_exists(&db_path)?;
    let pool = Traced::new(init(&db_path));

    /*
    let rt = Builder::new_current_thread().enable_all().build()?;
    rt.block_on(async {
        onetun::start_tunnels(config, bus).await?;
        api::run_api_server(pool).await?;
        Ok(())
    })
    */
    let local = LocalSet::new();
    log::debug!("Création du runtime tokio");
    let rt = Builder::new_current_thread().enable_all().build()?;

    //let _drop_sentinel = DropSentinel;
    
    /*
    local.block_on(&rt, async {
        onetun::start_tunnels(config, bus).await?;
        api::run_api_server(pool).await?;
        Ok(())
    })
    
    */
    log::debug!("Lancement du bloc tokio local");
    rt.block_on(local.run_until(async {
        log::debug!("Démarrage des tunnels onetun");
        onetun::start_tunnels(config, bus.inner).await?;

        log::debug!("Démarrage de l'API serveur");
        api::run_api_server(pool.inner).await?;

        Ok(())
    }))
}

// Affichage des logs côté Android Studio
pub fn init_android_logger() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("WirechatRust"),
    );

    log::debug!("Logger Android initialisé");
}

fn start_backend() {
    // Affichage des logs côté Android Studio
    init_android_logger();
    log::info!("Démarrage du backend Rust...");

    println!("Démarrage du backend Rust...");

    if let Err(e) = launch_backend() {
        log::error!("Erreur lors du lancement du backend : {}", e);
    } else {
        log::info!("Backend lancé avec succès");
    }
}

fn main() {
    start_backend();
}
