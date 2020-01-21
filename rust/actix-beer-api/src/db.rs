use crate::models::{Beer, Brewery};
use crate::schema::{beers, breweries};

use diesel;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;

use actix_web::web;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn find_beer_by_id(id: &str, pool: web::Data<Pool>) -> Option<Beer> {
    let conn: &SqliteConnection = &pool.get().unwrap();
    let beer = beers::table
        .find(id)
        .first(conn)
        .map_err(|err| eprintln!("beers::find_one: {}", err))
        .ok()?;
    Some(beer)
}

pub fn find_all_beer(pool: web::Data<Pool>) -> Option<Vec<Beer>> {
    let conn: &SqliteConnection = &pool.get().unwrap();
    beers::table
        .load::<Beer>(conn)
        .map_err(|err| eprintln!("beers::find_all_beer: {}", err))
        .ok()
}

pub fn find_all_beer_by_brewery(id: &str, pool: web::Data<Pool>) -> Option<Vec<Beer>> {
    let conn: &SqliteConnection = &pool.get().unwrap();
    beers::table
        .filter(beers::brewery_id.eq(id))
        .load::<Beer>(conn)
        .map_err(|err| eprintln!("beer::find_all_beer_by_brewery: {}", err))
        .ok()
}

pub fn find_brewery_by_id(id: &str, pool: web::Data<Pool>) -> Option<Brewery> {
    let conn: &SqliteConnection = &pool.get().unwrap();
    let brewery = breweries::table
        .find(id)
        .first(conn)
        .map_err(|err| eprintln!("beers::find_brewery_by_id: {}", err))
        .ok()?;
    Some(brewery)
}

pub fn find_all_breweries(pool: web::Data<Pool>) -> Option<Vec<Brewery>> {
    let conn: &SqliteConnection = &pool.get().unwrap();
    breweries::table
        .load::<Brewery>(conn)
        .map_err(|err| eprintln!("beers::find_all_breweries: {}", err))
        .ok()
}
