use diesel::{
    prelude::*,
    insert_into,
};
use serde::Serialize;

use crate::database::schema::peers;

// Struct correspondant à une ligne dans la table "peers"
#[derive(Queryable, Serialize, Debug)]
pub struct Peer {
    pub public_key: String,
    pub name: String,
    pub endpoint: String,
    pub allowed_ip: String,
    pub interface_ip: String,
}

// Struct utilisée pour insérer un nouveau peer dans la table
#[derive(Insertable)]
#[diesel(table_name = peers)]
pub struct NewPeer<'a> {
    pub public_key: &'a str,
    pub name: &'a str,
    pub endpoint: &'a str,
    pub allowed_ip: &'a str,
    pub interface_ip: &'a str,
}

// Fonction d'insertion dans la base
pub fn insert_peer(conn: &mut SqliteConnection, public_key: &str, name: &str, endpoint: &str, allowed_ip: &str, interface_ip: &str) {
    let new_peer = NewPeer {
        public_key,
        name,
        endpoint,
        allowed_ip,
        interface_ip,
    };

    let result = insert_into(peers::table)
        .values(&new_peer)
        .execute(conn);

    match result {
        Ok(n) => println!("Successfully inserted {} record(s).", n),
        Err(e) => println!("Error inserting peer: {}", e),
    }
}

// Fonction pour récupérer tous les peers
pub fn get_all_peers(conn: &mut SqliteConnection) -> QueryResult<Vec<Peer>> {
    peers::table.load::<Peer>(conn)
}
