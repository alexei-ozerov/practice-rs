extern crate futures;
extern crate hyper;

#[macro_use]
extern crate log;
extern crate env_logger;

use futures::future::Future;
use hyper::server::{Request, Response, Service};

struct Microservice;

// Create Service Impl for Microservice Struct
impl Service for Microservice {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        info!("Received a request: {:?}", request);
        Box::new(futures::future::ok(Response::new()))
    }
}

// Initialize Logging, Instantiate Service, Begin Listening
fn main() {
    env_logger::init();
    let address = "127.0.0.1:8080".parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&address, move || Ok(Microservice))
        .unwrap();

    info!("Running service at {}", address);
    server.run().unwrap();
}
