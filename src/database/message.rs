use diesel::{
    prelude::*,
    insert_into,
};

use chrono::NaiveDateTime;
use crate::database::schema::messages;

// Struct correspondant à une ligne dans la table "messages"
#[derive(Queryable)]
pub struct Message {
    pub id: Option<i32>,
    pub sender_public_key: String,
    pub receiver_public_key: String,
    pub message: String,
    pub timestamp: Option<NaiveDateTime>,
}

// Struct utilisée pour insérer un nouveau message dans la table
#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage<'a> {
    pub sender_public_key: &'a str,
    pub receiver_public_key: &'a str,
    pub message: &'a str,
    pub timestamp: Option<NaiveDateTime>,
}

// Fonction d'insertion dans la base
pub fn insert_message(
    conn: &mut SqliteConnection,
    sender_public_key: &str,
    receiver_public_key: &str,
    message: &str,
    timestamp: Option<chrono::NaiveDateTime>,
) {
    let new_message = NewMessage {
        sender_public_key,
        receiver_public_key,
        message,
        timestamp,
    };

    let result = insert_into(messages::table)
        .values(&new_message)
        .execute(conn);

    match result {
        Ok(n) => println!("Successfully inserted {} record(s).", n),
        Err(e) => println!("Error inserting message: {}", e),
    }
}

pub fn get_messages_for_peer(
    conn: &mut SqliteConnection,
    public_key: &str,
) -> QueryResult<Vec<Message>> {
    messages::table
        .filter(messages::receiver_public_key.eq(public_key))
        .load::<Message>(conn)
}

pub fn send_message(
    conn: &mut SqliteConnection,
    sender_public_key: &str,
    receiver_public_key: &str,
    message: &str,
) -> QueryResult<usize> {
    let new_message = NewMessage {
        sender_public_key,
        receiver_public_key,
        message,
        timestamp: None,
    };

    insert_into(messages::table)
        .values(&new_message)
        .execute(conn)
}

// Function to delete a message by ID
pub fn delete_message_by_id(conn: &mut SqliteConnection, message_id: i32) -> QueryResult<usize> {
    diesel::delete(messages::table.filter(messages::id.eq(message_id)))
        .execute(conn)
}

// Function to delete all messages for a specific peer
pub fn delete_messages_for_peer(
    conn: &mut SqliteConnection,
    public_key: &str,
) -> QueryResult<usize> {
    diesel::delete(messages::table.filter(messages::receiver_public_key.eq(public_key)))
        .execute(conn)
}
