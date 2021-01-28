extern crate ms;

use diesel;
use log::info;
use serde::Deserialize;
use serde_json::from_str;

use self::diesel::prelude::*;
use self::models::*;
use self::ms::*;

// Deserialize POST Request Payload JSON
#[derive(Deserialize, Debug)]
struct WritePayload {
    title: String,
    body: String,
}

// Retrieve Recent Practice Journal Entries
pub async fn show_journal() -> Result<Vec<(String, String, i32)>, hyper::Error> {
    use ms::schema::entries::dsl::*;

    let mut reponse_vec: Vec<(String, String, i32)> = Vec::new();
    let connection = establish_connection();
    let results = entries
        .limit(5)
        .load::<Entry>(&connection)
        .expect("Error loading posts");

    info!("Retrieved {} entries", results.len());

    for entry in results {
        reponse_vec.push((entry.title, entry.body, entry.id));
    }

    Ok(reponse_vec)
}

// Write Practice Journal Entry (via JSON payload)
pub async fn write_journal(payload: String) -> Result<(), hyper::Error> {
    let connection = establish_connection();

    let json_obj: WritePayload = from_str(&payload).unwrap();
    let title = json_obj.title;
    let body = json_obj.body;

    let post = create_post(&connection, &title, &body);
    info!("Saved record {} with id {}", title, post.id);
    Ok(())
}
