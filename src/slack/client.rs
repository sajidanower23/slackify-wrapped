use super::{bots::BotsAPI, emoji::EmojiAPI, reactions::ReactionsApi};

pub struct SlackClient {
    pub token: String,
    pub client: reqwest::Client,
}

impl SlackClient {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn emoji(&self) -> EmojiAPI {
        EmojiAPI {
            client: self.client.clone(),
            token: self.token.clone(),
        }
    }

    pub fn reactions(&self) -> ReactionsApi {
        ReactionsApi {
            client: self.client.clone(),
            token: self.token.clone(),
        }
    }

    pub fn bots(&self) -> BotsAPI {
        BotsAPI {
            client: self.client.clone(),
            token: self.token.clone(),
        }
    }
}
