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
    pract_date: String,
    title: String,
    body: String,
    notes: String,
    pract_time: i32,
    focus_time: i32,
}

// Retrieve Recent Practice Journal Entries
pub async fn show_journal() -> Result<Vec<(String, String, String, i32)>, hyper::Error> {
    use ms::schema::entries::dsl::*;

    let mut reponse_vec: Vec<(String, String, String, i32)> = Vec::new();
    let connection = establish_connection();
    let results = entries
        .limit(5)
        .load::<Entry>(&connection)
        .expect("Error loading posts");

    info!("Retrieved {} entries", results.len());

    for entry in results {
        info!("{:?}", entry.id);
        info!("{:?}", entry.title);
        reponse_vec.push((entry.pract_date, entry.title, entry.body, entry.id));
    }

    Ok(reponse_vec)
}

// Write Practice Journal Entry (via JSON payload)
pub async fn write_journal(payload: String) -> Result<(), hyper::Error> {
    let connection = establish_connection();

    let json_obj: WritePayload = from_str(&payload).unwrap();
    let title = json_obj.title;
    let body = json_obj.body;
    let notes = json_obj.notes;
    let pract_date = json_obj.pract_date;
    let pract_time = json_obj.pract_time;
    let focus_time = json_obj.focus_time;

    let post = create_post(
        &connection,
        &pract_date,
        &title,
        &body,
        &notes,
        &pract_time,
        &focus_time,
    );
    info!("Saved record {} with id {}", title, post.id);
    Ok(())
}
