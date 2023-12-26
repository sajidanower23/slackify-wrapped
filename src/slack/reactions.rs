use reqwest::Client;
use reqwest::Error;
use reqwest::Url;
use serde::Deserialize;
use serde_json::Value;

pub struct ReactionsApi {
    pub client: Client,
    pub token: String,
}

impl ReactionsApi {
    pub async fn add(&self, params: ReactionsAddParams) -> Result<ReactionsAddResponse, Error> {
        const URL: &str = "https://slack.com/api/reactions.add";
        let mut url = Url::parse(URL).expect("Unable to parse URL");
        url.query_pairs_mut()
            .append_pair("channel", &params.channel)
            .append_pair("name", &params.name)
            .append_pair("timestamp", &params.timestamp);

        let response = self
            .client
            .post(url.as_ref())
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        return match response.error_for_status() {
            Ok(response) => response.json::<Value>().await.map(|value| {
                match value.get("ok").unwrap().as_bool().unwrap() {
                    true => ReactionsAddResponse::Success(serde_json::from_value(value).unwrap()),
                    false => ReactionsAddResponse::Error(serde_json::from_value(value).unwrap()),
                }
            }),
            Err(error) => Err(error),
        };
    }
}

pub enum ReactionsAddResponse {
    Success(ReactionsAddSuccess),
    Error(ReactionsError),
}

#[derive(Deserialize)]
pub struct ReactionsAddSuccess {
    pub ok: bool,
}

#[derive(Deserialize)]
pub struct ReactionsError {
    pub ok: bool,
    pub error: String,
}

pub struct ReactionsAddParams {
    pub channel: String,
    pub name: String,
    pub timestamp: String,
}
