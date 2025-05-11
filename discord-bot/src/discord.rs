pub mod types;

use crate::discord::types::DiscordMessageResponse;
use reqwest::{Client, Error, header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DiscordMessage {
    content: String,
}

pub struct Discord<'a> {
    client: Client,
    channel_id: &'a str,
    base: &'a str,
}

impl<'a> Discord<'a> {
    pub fn new(token: &str, channel_id: &'a str) -> Self {
        let mut headers = header::HeaderMap::new();

        let auth_value = header::HeaderValue::from_str(format!("Bot {token}").as_str());

        headers.insert(
            header::AUTHORIZATION,
            auth_value.expect("Failed to set auth header value"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Something went wrong creating the HTTP client.");

        let base = "https://discord.com/api/v10";

        Self {
            client,
            base,
            channel_id,
        }
    }

    pub async fn send(&self, message: &str) -> Result<String, Error> {
        let base = self.base;
        let channel_id = self.channel_id;

        let msg = DiscordMessage {
            content: message.to_owned(),
        };

        let resp = self
            .client
            .post(format!("{base}/channels/{channel_id}/messages"))
            .json(&msg)
            .send()
            .await?
            .json::<DiscordMessageResponse>()
            .await?;

        Ok(resp.id)
    }

    pub async fn delete(&self, message_id: &str) -> Result<bool, Error> {
        let base = self.base;
        let channel_id = self.channel_id;

        let resp = self
            .client
            .delete(format!(
                "{base}/channels/{channel_id}/messages/{message_id}"
            ))
            .send()
            .await?;

        Ok(resp.status().is_success())
    }
}
