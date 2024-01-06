use crate::slack::util::add_param_to_url;
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

        add_param_to_url(&mut url, "channel", &params.channel);
        add_param_to_url(&mut url, "file", &params.file);
        add_param_to_url(&mut url, "file_comment", &params.file_comment);
        add_param_to_url(&mut url, "full", &params.full.map(|v| v.to_string()));
        add_param_to_url(&mut url, "timestamp", &params.timestamp);

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

    pub async fn list(&self, params: ReactionsListParams) -> Result<ReactionsListResponse, Error> {
        const URL: &str = "https://slack.com/api/reactions.list";
        let mut url = Url::parse(URL).expect("Unable to parse URL");

        add_param_to_url(&mut url, "count", &params.count.map(|v| v.to_string()));
        add_param_to_url(&mut url, "cursor", &params.cursor);
        add_param_to_url(&mut url, "full", &params.full.map(|v| v.to_string()));
        add_param_to_url(&mut url, "limit", &params.limit.map(|v| v.to_string()));
        add_param_to_url(&mut url, "page", &params.page.map(|v| v.to_string()));
        add_param_to_url(&mut url, "team_id", &params.team_id);
        add_param_to_url(&mut url, "user", &params.user);

        let response = self
            .client
            .get(url.as_ref())
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        return match response.error_for_status() {
            Ok(response) => response.json::<Value>().await.map(|value| {
                match value.get("ok").unwrap().as_bool().unwrap() {
                    false => ReactionsListResponse::Error(serde_json::from_value(value).unwrap()),
                    true => {
                        let items = value.get("items").unwrap().as_array().unwrap();
                        let mut reactions_list_items = Vec::new();
                        for item in items {
                            let item_type = item.get("type").unwrap().as_str().unwrap();
                            match item_type {
                                "message" => {
                                    reactions_list_items.push(
                                        ReactionsListItem::ReactionsListMessageItem {
                                            r#type: item_type.to_string(),
                                            channel: item
                                                .get("channel")
                                                .unwrap()
                                                .as_str()
                                                .unwrap()
                                                .to_string(),
                                            message: serde_json::from_value(
                                                item.get("message").unwrap().clone(),
                                            )
                                            .unwrap(),
                                        },
                                    );
                                }
                                "file" => {
                                    reactions_list_items.push(
                                        ReactionsListItem::ReactionsListFileItem {
                                            r#type: item_type.to_string(),
                                            file: serde_json::from_value(
                                                item.get("file").unwrap().clone(),
                                            )
                                            .unwrap(),
                                        },
                                    );
                                }
                                "file_comment" => {
                                    reactions_list_items.push(
                                        ReactionsListItem::ReactionsListFileCommentItem {
                                            r#type: item_type.to_string(),
                                            file: serde_json::from_value(
                                                item.get("file").unwrap().clone(),
                                            )
                                            .unwrap(),
                                            comment: item.get("comment").unwrap().clone(),
                                        },
                                    );
                                }
                                unexpected_type => {
                                    panic!("Unexpected item type: {}", unexpected_type)
                                }
                            }
                        }
                        ReactionsListResponse::Success(ReactionsListSuccess {
                            ok: true,
                            items: reactions_list_items,
                            response_metadata: match serde_json::from_value(
                                value.get("response_metadata").unwrap().clone(),
                            ) {
                                Ok(metadata) => metadata,
                                Err(_) => ReactionsListResponseMetadata::default(),
                            },
                        })
                    }
                }
            }),
            Err(error) => Err(error),
        };
    }

    pub async fn remove(
        &self,
        params: ReactionsRemoveParams,
    ) -> Result<ReactionsRemoveResponse, Error> {
        const URL: &str = "https://slack.com/api/reactions.remove";
        let mut url = Url::parse(URL).expect("Unable to parse URL");

        add_param_to_url(&mut url, "channel", &params.channel);
        add_param_to_url(&mut url, "file", &params.file);
        add_param_to_url(&mut url, "file_comment", &params.file_comment);
        add_param_to_url(&mut url, "name", &Some(params.name));
        add_param_to_url(&mut url, "timestamp", &params.timestamp);

        let response = self
            .client
            .post(url.as_ref())
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        return match response.error_for_status() {
            Ok(response) => response.json::<Value>().await.map(|value| {
                match value.get("ok").unwrap().as_bool().unwrap() {
                    true => {
                        ReactionsRemoveResponse::Success(serde_json::from_value(value).unwrap())
                    }
                    false => ReactionsRemoveResponse::Error(serde_json::from_value(value).unwrap()),
                }
            }),
            Err(error) => Err(error),
        };
    }
}

#[derive(Debug, Deserialize)]
pub struct ReactionsError {
    pub ok: bool,
    pub error: String,
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Reaction {
    pub name: String,
    pub users: Vec<String>,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct MessageData {
    pub r#type: String,
    pub text: String,
    pub user: String,
    pub ts: String,
    pub team: Option<String>,
    pub reactions: Vec<Reaction>,
    pub permalink: String,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct ReactionsGetMessage {
    pub ok: bool,
    pub r#type: String,
    pub message: MessageData,
    pub channel: String,
}

#[derive(Debug, Deserialize)]
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

pub struct ReactionsListParams {
    pub count: Option<i32>,
    pub cursor: Option<String>,
    pub full: Option<bool>,
    pub limit: Option<i32>,
    pub page: Option<i32>,
    pub team_id: Option<String>, // Only relevant for org_level apps
    pub user: Option<String>,
}

impl Default for ReactionsListParams {
    fn default() -> Self {
        Self {
            count: None,
            cursor: None,
            full: None,
            limit: None,
            page: None,
            team_id: None,
            user: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum ReactionsListItem {
    ReactionsListMessageItem {
        r#type: String,
        channel: String,
        message: MessageData,
    },
    ReactionsListFileItem {
        r#type: String,
        file: FileData,
    },
    ReactionsListFileCommentItem {
        r#type: String,
        file: FileData,
        comment: Value,
    },
}

#[derive(Debug, Deserialize)]
pub struct ReactionsListResponseMetadata {
    pub next_cursor: String,
}

impl Default for ReactionsListResponseMetadata {
    fn default() -> Self {
        Self {
            next_cursor: "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ReactionsListSuccess {
    pub ok: bool,
    pub items: Vec<ReactionsListItem>,
    pub response_metadata: ReactionsListResponseMetadata,
}

pub enum ReactionsListResponse {
    Success(ReactionsListSuccess),
    Error(ReactionsError),
}

// A valid request will need to contain:
// - name, and
// - channel and timestamp, OR
// - file, OR
// - file_comment
pub struct ReactionsRemoveParams {
    pub name: String,
    pub channel: Option<String>,
    pub file: Option<String>,
    pub file_comment: Option<String>,
    pub timestamp: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReactionsRemoveSuccess {
    pub ok: bool,
}

pub enum ReactionsRemoveResponse {
    Success(ReactionsRemoveSuccess),
    Error(ReactionsError),
}
