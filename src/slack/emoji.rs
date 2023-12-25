use std::collections::HashMap;
use serde::Deserialize;
use reqwest::Client;
use reqwest::Url;
use reqwest::Error;

pub struct EmojiListParams {
    pub pretty: u8,
    pub include_categories: bool,
}

impl EmojiListParams {
    pub fn new_default() -> Self {
        Self {
            pretty: 1,
            include_categories: false,
        }
    }
}

pub struct EmojiAPI {
    pub client: Client,
    pub token: String,
}

impl EmojiAPI {
    pub async fn list (&self, params: Option<EmojiListParams>) -> Result<EmojiListResponse, Error> {
        const URL: &str = "https://slack.com/api/emoji.list";
        let mut url = Url::parse(URL).unwrap();
        let params = params.unwrap_or(EmojiListParams::new_default());
        url.query_pairs_mut()
            .append_pair("pretty", &params.pretty.to_string())
            .append_pair("include_categories", &params.include_categories.to_string());

        let response = self.client
            .get(url.as_ref())
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        return match response.error_for_status() {
            Ok(response) => response.json::<EmojiListResponse>().await,
            Err(error) => Err(error),
        };
    }
}

type EmojiName = String;
type EmojiUrl = String;

#[derive(Deserialize)]
pub struct EmojiListResponse {
    pub ok: bool,
    pub emoji: HashMap<EmojiName, EmojiUrl>,
    pub cache_ts: String,
    pub categories_version: Option<String>,
    pub categories: Option<Vec<Category>>,
}

#[derive(Deserialize)]
pub struct Category {
    pub name: String,
    pub emoji_names: Vec<EmojiName>,
}
