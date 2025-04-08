use diesel::{
    prelude::*,
    insert_into,
};

table! {
    /// Table
    peers (public_key) {
        public_key -> Text,
        name -> Nullable<Text>,
        /// ip + port
        endpoint -> Text,
        /// if ipv6 end-to-end : same as endpoint? (without port)
        allowed_ip -> Text,
    }
}

#[derive(Queryable)]
struct Peers {
    public_key: String,
    name: Option<String>,
    endpoint: String,
    allowed_ip: String,
}

// The insertable struct. Using &'a str as it references data (could also use String)
#[derive(Insertable)]
#[table_name = "peers"]
struct NewPeer<'a> {
    public_key: &'a str,
    name: Option<&'a str>,
    endpoint: &'a str,
    allowed_ip: &'a str,
}

fn insert_peer(conn: &mut SqliteConnection) {
    // Create a new peer record.
    let new_peer = NewPeer {
        public_key: "abcd1234",
        name: Some("Alice"),  // or use None if you want to leave it as NULL
        endpoint: "192.168.1.10:8000",
        allowed_ip: "192.168.1.10",
    };

    // Insert the new peer into the database.
    let result = insert_into(peers::table)
        .values(&new_peer)
        .execute(conn);

    match result {
        Ok(n) => println!("Successfully inserted {} record(s).", n),
        Err(e) => println!("Error inserting peer: {}", e),
    }
}
