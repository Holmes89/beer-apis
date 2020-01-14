#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use actix_web::{get, web, App, HttpResponse, HttpServer};
use serde::Serialize;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Serialize)]
struct Beer {
    id: i64,
    brewery_id: i64,
    name: String,
}

#[get("/beer/{id}")]
async fn beer_by_id(path: web::Path<(i64,)>, pool: web::Data<Pool>) -> HttpResponse {
    let b = Beer {
        id: path.0,
        name: String::from("test"),
        brewery_id: 1,
    };
    HttpResponse::Ok().json(b)
}

#[get("/beer/")]
async fn beer_all(pool: web::Data<Pool>) -> HttpResponse {
    let b: Vec<Beer> = vec![
        Beer {
            id: 1,
            name: String::from("test"),
            brewery_id: 1,
        },
        Beer {
            id: 2,
            name: String::from("test2"),
            brewery_id: 3,
        },
    ];
    HttpResponse::Ok().json(b)
}

#[derive(Serialize)]
struct Brewery {
    id: i64,
    name: String,
    address: String,
    city: String,
    state: String,
    code: String,
}

#[get("/brewery/{id}/beer/")]
async fn beer_by_breweries(path: web::Path<(i64,)>, pool: web::Data<Pool>) -> HttpResponse {
    let b: Vec<Beer> = vec![
        Beer {
            id: 1,
            name: String::from("test"),
            brewery_id: path.0,
        },
        Beer {
            id: 2,
            name: String::from("test2"),
            brewery_id: path.0,
        },
    ];
    HttpResponse::Ok().json(b)
}

#[get("/brewery/{id}")]
async fn brewery_by_id(path: web::Path<(i64,)>, pool: web::Data<Pool>) -> HttpResponse {
    let b = Brewery {
        id: path.0,
        name: String::from("test"),
        address: String::from("123 pine street"),
        city: String::from("pittsburgh"),
        state: String::from("pa"),
        code: String::from("15238"),
    };
    HttpResponse::Ok().json(b)
}

#[get("/brewery/")]
async fn brewery_all(pool: web::Data<Pool>) -> HttpResponse {
    let b: Vec<Brewery> = vec![Brewery {
        id: 1,
        name: String::from("test"),
        address: String::from("123 pine street"),
        city: String::from("pittsburgh"),
        state: String::from("pa"),
        code: String::from("15238"),
    }];
    HttpResponse::Ok().json(b)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let manager = ConnectionManager::<SqliteConnection>::new("file:beers.db");
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
