#![deny(clippy::all)]

extern crate chrono;
extern crate env_logger;
extern crate iron;
extern crate logger;
extern crate router;
extern crate uuid;


use iron::prelude::*;
use iron::status;
use router::Router;
use logger::Logger;
use std::sync::Arc;

fn main() {
    env_logger::init();

    let mut router = Router::new();

    router.get("/", hello_world_handler, "index");
    router.get("/api", api_handler, "api");

    let (logger_before, logger_after) = Logger::new(None);
    let chain = Arc::new(Chain::new(router));
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let address = "127.0.0.1:3000";
    println!("Server running on http://{}", address);
    Iron::new(chain).http(address).unwrap();
}

fn hello_world_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello, world!")))
}

fn api_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, r#"{"message": "Welcome to the API"}"#)))
}
