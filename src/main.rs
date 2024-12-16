#![deny(clippy::all)]

extern crate chrono;
extern crate env_logger;
extern crate iron;
extern crate logger;
extern crate router;
extern crate uuid;

mod models;
mod databases;
mod handlers;

use models::*;
use databases::Database;
use handlers::*;
use iron::prelude::*;
use iron::Iron;
use router::Router;
use logger::Logger;
use uuid::Uuid;

fn main() {
    // Initialize the logger
    env_logger::init();

    // Initialize logging middleware
    let (logger_before, logger_after) = Logger::new(None);

    // Create a new database
    let mut db = Database::new();

    // Add posts to the database
    let p1 = Post::new(
        "The First Post",
        "This is the first post in our API",
        "Traybuns",
        chrono::Utc::now(),
        Uuid::new_v4(),
    );
    db.add_post(p1);

    let p2 = Post::new(
        "The next post is better",
        "Liquid Metal",
        "Indaboski",
        chrono::Utc::now(),
        Uuid::new_v4(),
    );
    db.add_post(p2);

    // Set up handlers
    let handlers = Handlers::new(db);
    let json_content_middleware = JsonAfterMiddleware;

    // Define routes
    let mut router = Router::new();
    router.get("/post_feed", handlers.post_feed, "post_feed");
    router.post("/post", handlers.post_post, "post_post");
    router.get("/post/:id", handlers.post, "post");

    // Set up middleware chain
    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(json_content_middleware);
    chain.link_after(logger_after);

    // Start the Iron server
    let address = "localhost:8000";
    println!("Server running on http://{}", address);
    Iron::new(chain).http(address).unwrap();
}
