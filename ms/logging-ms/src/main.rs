extern crate ms;

use diesel;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

use env_logger;
use log::{error, info};

use self::diesel::prelude::*;
use self::models::*;
use self::ms::*;

async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    // Routing
    match (req.method(), req.uri().path()) {
        // Return Message From DB
        (&Method::GET, "/") => {
            info!("Received GET Request: {:?}", req);
            let resp = show_journal();

            info!("Retreived Data: {:?}", resp);
            let latest_entry =
                "Title: ".to_owned() + &resp[resp.len() - 1].0 + "\nBody: " + &resp[resp.len() - 1].1;

            *response.body_mut() = Body::from(latest_entry);
        }

        // Write Data To DB
        (&Method::POST, "/write") => {
            info!("Received Request: {:?}", req);
            write_journal(req).await?;
            *response.body_mut() = Body::from("Written Data.");
        }

        // Return Echo Response
        (&Method::POST, "/test") => {
            info!("Received Request: {:?}", req);
            *response.body_mut() = req.into_body();
        }

        // Return Error
        _ => {
            error!("Received INVALID Request: {:?}", req);
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

fn show_journal() -> Vec<(String, String)> {
    use ms::schema::entries::dsl::*;

    let mut reponse_vec: Vec<(String, String)> = Vec::new();
    let connection = establish_connection();
    let results = entries
        .limit(5)
        .load::<Entry>(&connection)
        .expect("Error loading posts");

    info!("Retrieved {} entries", results.len());

    for entry in results {
        reponse_vec.push((entry.title, entry.body));
    }

    reponse_vec
}

async fn write_journal(request: Request<Body>) -> Result<(), hyper::Error> {
    let connection = establish_connection();

    let full_body = hyper::body::to_bytes(request.into_body()).await?;
    let string_bod = full_body.iter().cloned().collect::<Vec<u8>>();

    let title = String::from("Test!");
    let body = String::from_utf8_lossy(&string_bod).to_string();

    let post = create_post(&connection, &title, &body);
    info!("Saved record {} with id {}", title, post.id);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();

    let addr = ([127, 0, 0, 1], 3000).into();
    let svc = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(router)) });
    let server = Server::bind(&addr).serve(svc);

    println!("\n\n######################################");
    println!("Starting Server\nListening on http://{}", addr);
    println!("######################################\n");
    server.await?;

    Ok(())
}
