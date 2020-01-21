#![feature(proc_macro_hygiene, decl_macro)]

use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use rocket_contrib::json::Json;

mod db;
mod models;
mod schema;

use crate::db::*;
use crate::models::*;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;

#[database("beers.db")]
pub struct DbConn(SqliteConnection);

#[get("/beer", format = "json")]
fn find_all_beer(conn: DbConn) -> Json<Vec<Beer>> {
    Json(find_all_beer(pool).to_vec())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![find_all_beer])
        .attach(DbConn::fairing())
}

fn main() {
    rocket().launch();
}
