use std::collections::HashMap;
use reqwest::Error;
use serde::Deserialize;


type EmojiName = String;
type EmojiUrl = String;

#[derive(Deserialize)]
pub struct EmojiListResponse {
    pub ok: bool,
    pub emoji: HashMap<EmojiName, EmojiUrl>,
    pub cache_ts: String,
}

pub struct SlackClient {
    token: String,
    client: reqwest::Client,
}

impl SlackClient {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_emoji_list(&self) -> Result<EmojiListResponse, Error> {
        let url = "https://slack.com/api/emoji.list";
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;
        let emoji_list_response = response.json::<EmojiListResponse>().await?;
        Ok(emoji_list_response)
    }
}
