use std::collections::HashMap;
use serde::Deserialize;
use reqwest::Client;

pub struct EmojiAPI {
    pub client: Client,
    pub token: String,
}

impl EmojiAPI {
    pub async fn list (&self) -> Result<EmojiListResponse, reqwest::Error> {
        let url = "https://slack.com/api/emoji.list";
        let response = self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;
        let emoji_list_response = response.json::<EmojiListResponse>().await?;
        Ok(emoji_list_response)
    }
}

type EmojiName = String;
type EmojiUrl = String;

#[derive(Deserialize)]
pub struct EmojiListResponse {
    pub ok: bool,
    pub emoji: HashMap<EmojiName, EmojiUrl>,
    pub cache_ts: String,
}
