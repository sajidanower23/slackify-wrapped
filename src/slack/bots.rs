use reqwest::{Client, Error, Url};
use serde::Deserialize;
use serde_json::Value;

use crate::slack::util::add_param_to_url;

pub struct BotsInfoParams {
    pub bot: Option<String>,
    pub team_id: Option<String>,
}

impl BotsInfoParams {
    pub fn new_default() -> Self {
        Self {
            bot: None,
            team_id: None,
        }
    }
}

pub struct BotsAPI {
    pub client: Client,
    pub token: String,
}

impl BotsAPI {
    // https://api.slack.com/methods/bots.info
    pub async fn info(&self, params: Option<BotsInfoParams>) -> Result<BotsInfoResponse, Error> {
        const URL: &str = "https://slack.com/api/bots.info";
        let mut url = Url::parse(URL).expect("Unable to parse URL");
        let params = params.unwrap_or(BotsInfoParams::new_default());
        add_param_to_url(&mut url, "bot", &params.bot);
        add_param_to_url(&mut url, "team_id", &params.team_id);

        let response = self
            .client
            .get(url.as_ref())
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        return match response.error_for_status() {
            Ok(response) => response.json::<Value>().await.map(|value| {
                match value.get("ok").unwrap().as_bool().unwrap() {
                    true => BotsInfoResponse::Success(serde_json::from_value(value).unwrap()),
                    false => BotsInfoResponse::Error(serde_json::from_value(value).unwrap()),
                }
            }),
            Err(error) => Err(error),
        };
    }
}

pub enum BotsInfoResponse {
    Success(BotsInfoSuccess),
    Error(BotsInfoError),
}

#[derive(Deserialize)]
pub struct BotsInfoError {
    pub ok: bool,
    pub error: String,
}

#[derive(Deserialize)]
pub struct BotsInfoSuccess {
    pub ok: bool,
    pub bot: Bot,
}

#[derive(Deserialize)]
pub struct Bot {
    pub id: String,
    pub deleted: bool,
    pub name: String,
    pub app_id: String,
    pub user_id: String,
    pub updated: i64,
    pub icons: BotIcons,
}

#[derive(Deserialize)]
pub struct BotIcons {
    pub image_36: String,
    pub image_48: String,
    pub image_72: String,
}
