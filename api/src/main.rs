#[macro_use]
extern crate diesel;

extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;

//all the installation shit https://github.com/diesel-rs/diesel/issues/487#issuecomment-538719967
//things that I've done to make sure diesel works
//1. followed the link above (only installed postgres. try mysql prolly next time)
//2. make sure to have env_Var in the settings -> PQ_LIB_DIR points to Postgres/12/lib
//3. copied libpq.dll to .cargo/bin

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("Gravity's Rainbow"),
        author: String::from("Thomas Pynchon"),
        published: true,
    };

    if models::Book::insert(book, &conn) {
        println!("success");
    } else {
        println!("failed");
    }
}
