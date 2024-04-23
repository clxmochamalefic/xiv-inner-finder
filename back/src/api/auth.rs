use serenity::prelude::Client;

async fn login() -> Client {
    let token = std::env::var("DISCORD_TOKEN")?;
    let mut client = Client::builder(&token, GatewayIntents::default()).await?;

    if let Err(why) = client.start().await {
        println!("Err with client: {:?}", why)
    }

    return client;
}
