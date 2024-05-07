use actix_web::{get, HttpResponse, Responder};
use reqwest;
use serde_json::value::Value as SerdeValue;
use xivapi_rust::{Language, XIVClient};

use crate::store::secrets::Secrets;

const BASEURL: &str = "https://xivapi.com/";
const BASIC_INFO_PATH: &str = "item/1675";

#[get("/xiv/info")]
pub async fn xiv_info() -> impl Responder {
    println!("=== info ===");
    let secrets = Secrets::new();
    let secret = secrets.get_secret("xivapi_secret");

    let uri = format!(
        "{}{}?{}{}",
        BASEURL, BASIC_INFO_PATH, "private_key=", secret
    );
    let json: String = reqwest::get(&uri).await.unwrap().text().await.unwrap();

    HttpResponse::Ok().json(json)
}

#[get("/xiv/user")]
pub async fn xiv_user() -> impl Responder {
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
