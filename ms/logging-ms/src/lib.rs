#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{Entry, NewEntry};

pub mod models;
pub mod schema;

// Return Connection
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// Write Data To Database
pub fn create_post<'a>(conn: &PgConnection, pract_date: &'a str, title: &'a str, body: &'a str, notes: &'a str, pract_time: &i32, focus_time: &i32) -> Entry {
    use schema::entries;

    let new_entry = NewEntry {
        pract_date: pract_date,
        title: title,
        body: body,
        notes: notes,
        pract_time: pract_time,
        focus_time: focus_time,
    };

    diesel::insert_into(entries::table)
        .values(&new_entry)
        .get_result(conn)
        .expect("Error saving new post")
}
