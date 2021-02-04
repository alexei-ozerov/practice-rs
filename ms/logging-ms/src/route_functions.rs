extern crate ms;

use diesel;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_json::{json, to_string_pretty};

use self::diesel::prelude::*;
use self::models::*;
use self::ms::*;

// Deserialize POST Request Payload JSON
#[derive(Deserialize, Debug)]
struct WritePayload {
    pract_date: String,
    title: String,
    goal: String,
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
        .order_by(id.desc())
        .load::<Entry>(&connection)
        .expect("Error loading posts");

    info!("Retrieved {} entries", results.len());

    for entry in results {
        info!("{:?}", entry.id);
        info!("{:?}", entry.title);
        reponse_vec.push((entry.pract_date, entry.title, entry.goal, entry.id));
    }

    Ok(reponse_vec)
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct JournalData {
    date: Vec<String>,
    title: Vec<String>,
    goal: Vec<String>,
}

// Format the Response Object
pub async fn return_journal(
    resp: Vec<(String, String, String, i32)>,
) -> Result<String, hyper::Error> {
    // Instantiate Vecs
    let mut dateVec: Vec<String> = Vec::new();
    let mut titleVec: Vec<String> = Vec::new();
    let mut goalVec: Vec<String> = Vec::new();

    // Push Data
    for t in resp.into_iter() {
        dateVec.push(t.0);
        titleVec.push(t.1);
        goalVec.push(t.2);
    }

    // Instantiate Struct
    let map = JournalData {
        date: dateVec,
        title: titleVec,
        goal: goalVec,
    };

    // Convert to Json String
    let json_string = json!(map);
    let resp_string = to_string_pretty(&json_string).unwrap();
    Ok(resp_string)
}

// Write Practice Journal Entry (via JSON payload)
pub async fn write_journal(payload: String) -> Result<(), hyper::Error> {
    let connection = establish_connection();

    let json_obj: WritePayload = from_str(&payload).unwrap();
    let title = json_obj.title;
    let goal = json_obj.goal;
    let notes = json_obj.notes;
    let pract_date = json_obj.pract_date;
    let pract_time = json_obj.pract_time;
    let focus_time = json_obj.focus_time;

    let post = create_post(
        &connection,
        &pract_date,
        &title,
        &goal,
        &notes,
        &pract_time,
        &focus_time,
    );
    info!("Saved record {} with id {}", title, post.id);
    Ok(())
}
