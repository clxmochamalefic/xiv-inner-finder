use serenity::client::*;
use serenity::prelude::*;

use crate::store::secrets::Secrets;

async fn login() -> Client {
    let secrets = Secrets::new();
    let token = secrets.get_secret("discord_secret");
    let mut client: Client = Client::builder(&token, GatewayIntents::default())
        .await
        .unwrap();

    if let Err(why) = client.start().await {
        println!("Err with client: {:?}", why)
    }

    return client;
}
