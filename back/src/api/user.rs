use actix_web::{get, HttpResponse, Responder};
use serde_json::value::Value as SerdeValue;
use xivapi_rust::{Language, XIVClient};

use crate::store::secrets::Secrets;

#[get("/user")]
pub async fn index() -> impl Responder {
    println!("=== index ===");
    let secrets = Secrets::new();
    let client = XIVClient::new(
        secrets.get_secret("xivapi_secret"),
        Language::Japanese,
        Some(true),
        Some(true),
        Some(true),
    );
    println!("=== get chara ===");
    let character: SerdeValue = client
        .character()
        .search("Cocoalix Mochamalefic", Some("Atomos"), Some(1))
        .await
        .unwrap();
    let json: &str = character.as_str().unwrap();
    println!("{}", json);
    HttpResponse::Ok().json(json)
}
