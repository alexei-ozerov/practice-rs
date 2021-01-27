extern crate ms;

use diesel;
use env_logger;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use log::{error, info};
use serde::Deserialize;
use serde_json::{from_str, json, to_string_pretty, Map, Value};

use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;

use self::diesel::prelude::*;
use self::models::*;
use self::ms::*;

// Deserialize POST Request Payload JSON
#[derive(Deserialize, Debug)]
struct WritePayload {
    title: String,
    body: String,
}

/*          ###############
            Practice RS API
                ROUTES
            ###############

   /        => Get Most Recent Journal Entries
   /write   => Add New Journal Entry
   /health  => Return Message & Machine Timestamp  */
async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        // Return Message From DB
        (&Method::GET, "/") => {
            info!("Received GET Request: {:?}", req);
            let resp = show_journal().await?;

            let mut map = Map::new();
            for t in resp.into_iter() {
                map.insert("Title_".to_owned() + &t.2.to_string(), Value::String(t.0));
                map.insert("Body_".to_owned() + &t.2.to_string(), Value::String(t.1));
            }

            let json_string = json!(map);
            let resp_string = to_string_pretty(&json_string).unwrap();

            info!("Retreived Data: {:?}", resp_string);
            *response.body_mut() = Body::from(resp_string)
        }

        // Write Data To DB
        (&Method::POST, "/write") => {
            info!("Received Request: {:?}", req);

            // Convert Request To Bytes To JSON, with Error on Failure
            let bytes = hyper::body::to_bytes(req.into_body()).await?;
            let payload = String::from_utf8(bytes.to_vec()).unwrap();

            write_journal(payload).await?;
            *response.body_mut() = Body::from("Written Data.")
        }

        // Return Echo Response
        (&Method::GET, "/health") => {
            info!("Received Request: {:?}", req);
            let system_time = SystemTime::now();
            let datetime: DateTime<Utc> = system_time.into();
            *response.body_mut() =
                Body::from("The API is active | ".to_owned() + &datetime.to_string())
        }

        // Return Error
        _ => {
            error!("Received INVALID Request: {:?}", req);
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

// Retrieve Recent Practice Journal Entries
async fn show_journal() -> Result<Vec<(String, String, i32)>, hyper::Error> {
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
async fn write_journal(payload: String) -> Result<(), hyper::Error> {
    let connection = establish_connection();

    let json_obj: WritePayload = from_str(&payload).unwrap();
    let title = json_obj.title;
    let body = json_obj.body;

    let post = create_post(&connection, &title, &body);
    info!("Saved record {} with id {}", title, post.id);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();

    let addr = ([0, 0, 0, 0], 3000).into();
    let svc = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(router)) });
    let server = Server::bind(&addr).serve(svc);

    println!("\n\n###################################");
    println!("Starting Server\nListening on http://{}", addr);
    println!("###################################\n");
    server.await?;

    Ok(())
}
