use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;

use actix_web::{get, web, App, HttpResponse, HttpServer, ResponseError};

mod db;
mod models;
mod schema;

#[macro_use]
extern crate diesel;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/beer/{id}")]
async fn beer_by_id(path: web::Path<(String,)>, pool: web::Data<Pool>) -> HttpResponse {
    let id = path.0.as_ref();
    let results = db::find_one(id, pool);
    HttpResponse::Ok().json(results)
}

#[get("/beer/")]
async fn beer_all(pool: web::Data<Pool>) -> HttpResponse {
    HttpResponse::Ok().json("not implemented")
}

#[get("/brewery/{id}/beer/")]
async fn beer_by_breweries(path: web::Path<(String,)>, pool: web::Data<Pool>) -> HttpResponse {
    HttpResponse::Ok().json("not implemented")
}

#[get("/brewery/{id}")]
async fn brewery_by_id(path: web::Path<(String,)>, pool: web::Data<Pool>) -> HttpResponse {
    HttpResponse::Ok().json("not implemented")
}

#[get("/brewery/")]
async fn brewery_all(pool: web::Data<Pool>) -> HttpResponse {
    HttpResponse::Ok().json("not implemented")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let manager = ConnectionManager::<SqliteConnection>::new("beers.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(beer_by_id)
            .service(beer_all)
            .service(beer_by_breweries)
            .service(brewery_by_id)
            .service(brewery_all)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
