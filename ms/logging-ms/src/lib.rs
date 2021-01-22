#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::{NewEntry, Entry};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Entry {
    use schema::entries;

    let new_entry = NewEntry {
        title: title,
        body: body,
    };

    diesel::insert_into(entries::table)
        .values(&new_entry)
        .get_result(conn)
        .expect("Error saving new post")
}
