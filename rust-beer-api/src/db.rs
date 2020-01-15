use crate::models::Beer;
use crate::schema::beers;

use diesel;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;

use actix_web::web;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn find_one(id: &str, pool: web::Data<Pool>) -> Option<Beer> {
    let conn: &SqliteConnection = &pool.get().unwrap();
    let beer = beers::table
        .find(id)
        .first(conn)
        .map_err(|err| eprintln!("beers::find_one: {}", err))
        .ok()?;
    Some(beer)
}
