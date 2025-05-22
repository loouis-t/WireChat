use diesel::{
    prelude::*,
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

// Fonction pour récupérer tous les peers
pub fn get_all_peers(conn: &mut SqliteConnection) -> QueryResult<Vec<Peer>> {
    peers::table.load::<Peer>(conn)
}
