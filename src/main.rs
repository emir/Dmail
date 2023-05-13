extern crate yup_oauth2 as oauth2;
extern crate reqwest;
extern crate serde_json;
extern crate serde;
extern crate dotenv;
extern crate serenity;

use oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use reqwest::header;
use serde::{Deserialize};
use dotenv::dotenv;
use std::env;
use serenity::model::id::ChannelId;
use serenity::client::bridge::gateway::{ShardManager};
use serenity::http::Http;
use serenity::prelude::*;

#[derive(Deserialize)]
struct ListResponse {
    messages: Vec<MailMessage>,
}

#[derive(Deserialize)]
struct MailMessage {
    id: String,
    threadId: String,
}

#[derive(Deserialize)]
struct MessageResponse {
    id: String,
    snippet: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let secret_file = env::var("SECRET_FILE").expect("SECRET_FILE must be set");
    let secret = oauth2::read_application_secret(secret_file)
        .await
        .expect("client_secret.json");

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .build()
        .await
        .expect("InstalledFlowAuthenticator failed to build");

    let token = auth
        .token(&["https://www.googleapis.com/auth/gmail.readonly"])
        .await
        .expect("Failed to get token");

    let client = reqwest::Client::new();

    let bearer = format!("Bearer {}", token.as_str());
    let list_request = client
        .get("https://www.googleapis.com/gmail/v1/users/me/messages")
        .header(header::AUTHORIZATION, bearer);

    let response = list_request.send().await?;

    let list_response: ListResponse = serde_json::from_str(&response.text().await?)?;

    let discord_token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");

    let http = Http::new_with_token(&discord_token);
    let channel_id = ChannelId(env::var("CHANNEL_ID").expect("CHANNEL_ID must be set").parse::<u64>()?);

    for message in list_response.messages {
        let token = auth.token(&["https://www.googleapis.com/auth/gmail.readonly"]).await?;
        let bearer = format!("Bearer {}", token.as_str());
        let message_request = client
            .get(&format!("https://www.googleapis.com/gmail/v1/users/me/messages/{}", message.id))
            .header(header::AUTHORIZATION, bearer);

        let message_response = message_request.send().await?;

        let message_data: MessageResponse = serde_json::from_str(&message_response.text().await?)?;

        let channel_id = ChannelId(env::var("CHANNEL_ID").expect("CHANNEL_ID must be set").parse::<u64>()?);

        channel_id.say(&http, format!("New email: {}", message_data.snippet)).await?;
    }

    Ok(())
}
