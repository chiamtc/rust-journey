extern crate chrono;
extern crate env_logger;
extern crate iron;
extern crate logger;
extern crate router;
extern crate rustc_serialize;
extern crate uuid;

mod models;
mod database;
mod handlers;

use models::*;
use database::Database;
use handlers::*;

use iron::prelude::Chain;
use iron::Iron;
use router::Router;
use logger::Logger;
use uuid::Uuid;

fn main() {
    env_logger::init();
    let (logger_before, logger_after) = Logger::new(None);

    let mut db = Database::new();

    let p = Post::new(
        "The first post",
        "This is the fuirst post",
        "TC",
        chrono::offset::Utc::now(),
        Uuid::new_v4(),
    );
    db.add_post(p);
    let p2 = Post::new(
        "Second post",
        "Second post description",
        "TC",
        chrono::offset::Utc::now(),
        Uuid::new_v4(),
    );
    db.add_post(p2);

    let handlers = Handlers::new(db);
    let json_content_middleware = JsonAfterMiddleware;
    let mut router = Router::new();
    router.get("/post_feed", handlers.post_feed, "post_feed");
    router.get("/post", handlers.post_post, "post_post");
    router.get("/post/:id", handlers.post, "post");

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(json_content_middleware);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:8000");
}
