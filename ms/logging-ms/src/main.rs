extern crate dotenv;
extern crate ms;
mod route_functions;

use chrono::offset::Utc;
use chrono::DateTime;
use dotenv::dotenv;
use env_logger;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use log::{error, info};

use std::env;
use std::time::SystemTime;

/*#######################################################
    Practice RS API ROUTES:

    /        => Get Most Recent Journal Entries
    /write   => Add New Journal Entry
    /health  => Return Message & Machine Timestamp
#######################################################*/
async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(Body::empty())
        .unwrap();
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
            let resp_string = route_functions::return_journal(resp).await?;

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

        // Handle OPTIONS requests
        (&Method::OPTIONS, "/health") => {
            info!("Received OPTIONS Request: {:?}", req);
            *response.status_mut() = StatusCode::OK;
        }

        (&Method::OPTIONS, "/recent") => {
            info!("Received OPTIONS Request: {:?}", req);
            *response.status_mut() = StatusCode::OK;
        }

        (&Method::OPTIONS, "/write") => {
            info!("Received OPTIONS Request: {:?}", req);
            *response.status_mut() = StatusCode::OK;
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
    dotenv().ok();

    let addr_string = env::var("MS_ADDR").expect("MS_ADDR must be set!");
    let address = addr_string.parse().unwrap();

    let svc = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(router)) });
    let server = Server::bind(&address).serve(svc);

    println!(
        "\n\n
        ###################################\n
        Starting Server
        Listening on http://{}\n
        ###################################\n",
        address
    );

    server.await?;
    Ok(())
}
