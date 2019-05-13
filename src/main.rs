#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::models::*;

fn main() {
    use self::schema::words::dsl::*;
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env varible must be set");
    let conn = MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url));
    let results = words
        .limit(5)
        .load::<Word>(&conn)
        .expect("Error loading words");
    for w in results {
        dbg!(w);
    }
}
