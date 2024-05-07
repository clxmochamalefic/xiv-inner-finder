use actix_web::{post, HttpResponse, Responder};
use serenity::client::*;
use serenity::prelude::*;

use crate::store::secrets::Secrets;

#[post("/login")]
pub async fn login() -> impl Responder {
    let secrets = Secrets::new();
    let token = secrets.get_secret("discord_secret");
    let mut client: Client = Client::builder(&token, GatewayIntents::default())
        .await
        .unwrap();

    if let Err(why) = client.start().await {
        println!("Err with client: {:?}", why);
        return HttpResponse::NonAuthoritativeInformation();
    }

    HttpResponse::Ok()
}
