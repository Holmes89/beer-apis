use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;

#[derive(Serialize)]
struct Beer {
    id: i64,
    brewery_id: i64,
    name: String,
}

impl Responder for Beer {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[get("/beer/{id}")]
async fn beer(path: web::Path<(i64,)>) -> impl Responder {
    Beer {
        id: path.0,
        name: String::from("test"),
        brewery_id: 1,
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(beer))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
