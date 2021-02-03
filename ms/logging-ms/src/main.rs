extern crate ms;
mod route_functions;

use chrono::offset::Utc;
use chrono::DateTime;
use env_logger;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use log::{error, info};
use serde_json::{json, to_string_pretty, Map, Value};
use std::time::SystemTime;

/*#######################################################
    Practice RS API ROUTES:

    /        => Get Most Recent Journal Entries
    /write   => Add New Journal Entry
    /health  => Return Message & Machine Timestamp
#######################################################*/
async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        // Base Route
        (&Method::GET, "/") => {
            info!("Received GET Request: {:?}", req);
            *response.body_mut() = Body::from("Welcome to the Practice Journal Application. Please visit the appropriate route if using CURL to read / write data.")
        }

        // Return Message From DB
        (&Method::GET, "/recent") => {
            info!("Received GET Request: {:?}", req);
            let resp = route_functions::show_journal().await?;

            let mut map = Map::new();
            for t in resp.into_iter() {
                map.insert("Date_".to_owned() + &t.3.to_string(), Value::String(t.0));
                map.insert("Title_".to_owned() + &t.3.to_string(), Value::String(t.1));
                map.insert("Goal_".to_owned() + &t.3.to_string(), Value::String(t.2));
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

            route_functions::write_journal(payload).await?;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();

    let addr = ([0, 0, 0, 0], 3000).into();
    let svc = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(router)) });
    let server = Server::bind(&addr).serve(svc);

    println!(
        "\n\n
        ###################################\n
        Starting Server
        Listening on http://{}\n
        ###################################\n",
        addr
    );

    server.await?;
    Ok(())
}
