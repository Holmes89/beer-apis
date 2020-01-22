#![feature(proc_macro_hygiene, decl_macro)]

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use rocket_contrib::json::Json;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

use serde_json::json;

use std::ops::Deref;

mod models;
mod schema;

use crate::models::*;
use crate::schema::*;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new("beers.db");
    r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create pool")
}

pub struct DbConn(r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn error_status(error: Error) -> Status {
    eprintln!("db error {}", error);
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/beer")]
fn find_all_beer(conn: DbConn) -> Result<Json<Vec<Beer>>, Status> {
    beers::table
        .load::<Beer>(&*conn)
        .map(|beer| Json(beer))
        .map_err(|err| error_status(err))
}

#[get("/beer/<id>")]
fn find_beer_by_id(id: String, conn: DbConn) -> Result<Json<Beer>, Status> {
    beers::table
        .find(id)
        .first(&*conn)
        .map(|beer| Json(beer))
        .map_err(|err| error_status(err))
}

#[get("/brewery")]
fn find_all_breweries(conn: DbConn) -> Result<Json<Vec<Brewery>>, Status> {
    breweries::table
        .load::<Brewery>(&*conn)
        .map(|brewery| Json(brewery))
        .map_err(|err| error_status(err))
}

#[get("/brewery/<id>")]
fn find_brewery_by_id(id: String, conn: DbConn) -> Result<Json<Brewery>, Status> {
    breweries::table
        .find(id)
        .first(&*conn)
        .map(|brewery| Json(brewery))
        .map_err(|err| error_status(err))
}

#[get("/brewery/<id>/beer")]
fn find_brewery_beers(id: String, conn: DbConn) -> Result<Json<Vec<Beer>>, Status> {
    beers::table
        .filter(beers::brewery_id.eq(id))
        .load::<Beer>(&*conn)
        .map(|beer| Json(beer))
        .map_err(|err| error_status(err))
}

#[catch(404)]
fn not_found(_req: &Request) -> Json<serde_json::Value> {
    Json(json!({"status": "not found"}))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                find_all_beer,
                find_beer_by_id,
                find_all_breweries,
                find_brewery_by_id,
                find_brewery_beers
            ],
        )
        .register(catchers![not_found])
        .manage(init_pool())
}

fn main() {
    rocket().launch();
}
