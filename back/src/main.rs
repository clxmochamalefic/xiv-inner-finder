use actix_web::{get, web, App, HttpServer, Responder};

mod api;
mod store;
use crate::api::user::index as user_index;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[tokio::main]
//#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet).service(user_index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
