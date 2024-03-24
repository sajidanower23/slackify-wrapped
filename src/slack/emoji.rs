use reqwest::Client;
use reqwest::Error;
use reqwest::Url;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

pub struct EmojiListParams {
    pub include_categories: bool,
}

impl EmojiListParams {
    pub fn new_default() -> Self {
        Self {
            include_categories: false,
        }
    }
}

pub struct EmojiAPI {
    pub client: Client,
    pub token: String,
}

impl EmojiAPI {
    // https://api.slack.com/methods/emoji.list
    pub async fn list(&self, params: Option<EmojiListParams>) -> Result<EmojiListResponse, Error> {
        const URL: &str = "https://slack.com/api/emoji.list";
        let mut url = Url::parse(URL).expect("Unable to parse URL");
        let params = params.unwrap_or(EmojiListParams::new_default());
        url.query_pairs_mut()
            .append_pair("include_categories", &params.include_categories.to_string());

        let response = self
            .client
            .get(url.as_ref())
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        return match response.error_for_status() {
            Ok(response) => response.json::<Value>().await.map(|value| {
                match value.get("ok").unwrap().as_bool().unwrap() {
                    true => EmojiListResponse::Success(serde_json::from_value(value).unwrap()),
                    false => EmojiListResponse::Error(serde_json::from_value(value).unwrap()),
                }
            }),
            Err(error) => Err(error),
        };
    }
}

type EmojiName = String;
type EmojiUrl = String;

pub enum EmojiListResponse {
    Success(EmojiListSuccess),
    Error(EmojiError),
}

#[derive(Deserialize)]
pub struct EmojiError {
    pub ok: bool,
    pub error: String,
}

#[derive(Deserialize)]
pub struct EmojiListSuccess {
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
