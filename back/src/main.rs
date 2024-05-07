use actix_web::{get, web, App, HttpServer, Responder};

mod api;
mod store;
use crate::api::auth::login;
use crate::api::user::index as user_index;

#[get("/")]
async fn root() -> impl Responder {
    format!("Hello world")
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[tokio::main]
//#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello world");
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(greet)
            .service(user_index)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
