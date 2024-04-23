use actix_web::{get, HttpResponse, Responder};
use xivapi_rust::XIVClient;

#[get("/user")]
async fn index() -> impl Responder {
    Ok(HttpResponse::Ok().json("Hello world"))
}
