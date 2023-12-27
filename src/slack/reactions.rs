use reqwest::Client;
use reqwest::Error;
use reqwest::Url;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

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

    pub async fn get(&self, params: ReactionsGetParams) -> Result<ReactionsGetResponse, Error> {
        const URL: &str = "https://slack.com/api/reactions.get";
        let mut url = Url::parse(URL).expect("Unable to parse URL");
        let mut add_param = |name: &str, value: &Option<String>| {
            if let Some(val) = value {
                url.query_pairs_mut().append_pair(name, val);
            }
        };

        add_param("channel", &params.channel);
        add_param("file", &params.file);
        add_param("file_comment", &params.file_comment);
        add_param("full", &params.full.map(|v| v.to_string()));
        add_param("timestamp", &params.timestamp);

        let response = self
            .client
            .get(url.as_ref())
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        return match response.error_for_status() {
            Ok(response) => response.json::<Value>().await.map(|value| {
                match value.get("ok").unwrap().as_bool().unwrap() {
                    false => ReactionsGetResponse::Error(serde_json::from_value(value).unwrap()),
                    true => match value.get("type").unwrap().as_str().unwrap() {
                        "message" => ReactionsGetResponse::Success(
                            ReactionsGetSuccess::ReactionsGetMessage {
                                ok: true,
                                r#type: value.get("type").unwrap().as_str().unwrap().to_string(),
                                message: serde_json::from_value(
                                    value.get("message").unwrap().clone(),
                                )
                                .unwrap(),
                                channel: value
                                    .get("channel")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .to_string(),
                            },
                        ),
                        "file" => {
                            ReactionsGetResponse::Success(ReactionsGetSuccess::ReactionsGetFile {
                                ok: true,
                                r#type: value.get("type").unwrap().as_str().unwrap().to_string(),
                                file: serde_json::from_value(value.get("file").unwrap().clone())
                                    .unwrap(),
                            })
                        }
                        "file_comment" => ReactionsGetResponse::Success(
                            ReactionsGetSuccess::ReactionsGetFileComment {
                                ok: true,
                                r#type: value.get("type").unwrap().as_str().unwrap().to_string(),
                                file: serde_json::from_value(value.get("file").unwrap().clone())
                                    .unwrap(),
                                comment: value.get("comment").unwrap().clone(),
                            },
                        ),
                        _ => panic!("Unexpected type"),
                    },
                }
            }),
            Err(error) => Err(error),
        };
    }
}

pub struct ReactionsAddParams {
    pub channel: String,
    pub name: String,
    pub timestamp: String,
}

pub enum ReactionsAddResponse {
    Success(ReactionsAddSuccess),
    Error(ReactionsError),
}

#[derive(Deserialize)]
pub struct ReactionsAddSuccess {
    pub ok: bool,
}

pub struct ReactionsGetParams {
    pub channel: Option<String>,
    pub file: Option<String>,
    pub file_comment: Option<String>,
    pub full: Option<bool>,
    pub timestamp: Option<String>,
}

#[derive(Deserialize)]
pub struct Reaction {
    pub name: String,
    pub users: Vec<String>,
    pub count: i32,
}

#[derive(Deserialize)]
pub struct MessageData {
    pub r#type: String,
    pub text: String,
    pub user: String,
    pub ts: String,
    pub team: String,
    pub reactions: Vec<Reaction>,
    pub permalink: String,
}

#[derive(Deserialize)]
pub struct FileData {
    pub id: String,
    pub created: i32,
    pub timestamp: i32,
    pub name: String,
    pub title: String,
    pub mimetype: String,
    pub filetype: String,
    pub pretty_type: String,
    pub user: String,
    pub user_team: String,
    pub editable: bool,
    pub size: i32,
    pub mode: String,
    pub is_external: bool,
    pub external_type: String,
    pub is_public: bool,
    pub public_url_shared: bool,
    pub display_as_bot: bool,
    pub username: String,
    pub url_private: String,
    pub url_private_download: String,
    pub permalink: String,
    pub permalink_public: String,
    pub edit_link: String,
    pub preview: String,
    pub preview_highlight: String,
    pub lines: i32,
    pub lines_more: i32,
    pub preview_is_truncated: bool,
    pub comments_count: i32,
    pub is_starred: bool,
    pub shares: HashMap<String, Value>,
    pub channels: Vec<String>,
    pub groups: Vec<String>,
    pub ims: Vec<String>,
    pub has_more_shares: bool,
    pub has_rich_preview: bool,
    pub file_access: String,
}

#[derive(Deserialize)]
pub struct ReactionsGetMessage {
    pub ok: bool,
    pub r#type: String,
    pub message: MessageData,
    pub channel: String,
}

#[derive(Deserialize)]
pub enum ReactionsGetSuccess {
    ReactionsGetMessage {
        ok: bool,
        r#type: String,
        message: MessageData,
        channel: String,
    },
    ReactionsGetFile {
        ok: bool,
        r#type: String,
        file: FileData,
    },
    ReactionsGetFileComment {
        ok: bool,
        r#type: String,
        file: FileData,
        comment: Value,
    },
}

pub enum ReactionsGetResponse {
    Success(ReactionsGetSuccess),
    Error(ReactionsError),
}

#[derive(Deserialize)]
pub struct ReactionsError {
    pub ok: bool,
    pub error: String,
}
