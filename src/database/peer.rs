use diesel::{
    prelude::*,
    insert_into,
};

use crate::database::schema::peers;


// Struct correspondant à une ligne dans la table "peers"
#[allow(dead_code)]
#[derive(Queryable)]
pub struct Peer {
    pub public_key: String,
    pub name: Option<String>,
    pub endpoint: String,
    pub allowed_ip: String,
}

impl Drop for Peer {
    fn drop(&mut self) {
        log::warn!("Peer est en cours de drop (!)");
    }
}

// Struct utilisée pour insérer un nouveau peer dans la table
#[allow(dead_code)]
#[derive(Insertable)]
#[diesel(table_name = peers)]
pub struct NewPeer<'a> {
    pub public_key: &'a str,
    pub name: Option<&'a str>,
    pub endpoint: &'a str,
    pub allowed_ip: &'a str,
}

impl<'a> Drop for NewPeer<'a> {
    fn drop(&mut self) {
        log::warn!("NewPeer<'a> est en cours de drop (!)");
    }
}

// Fonction d'insertion dans la base
#[allow(dead_code)]
pub fn insert_peer(conn: &mut SqliteConnection, public_key: &str, name: Option<&str>, endpoint: &str, allowed_ip: &str) {
    let new_peer = NewPeer {
        public_key,
        name,
        endpoint,
        allowed_ip,
    };

    let result = insert_into(peers::table)
        .values(&new_peer)
        .execute(conn);

    match result {
        Ok(n) => println!("Successfully inserted {} record(s).", n),
        Err(e) => println!("Error inserting peer: {}", e),
    }
}
