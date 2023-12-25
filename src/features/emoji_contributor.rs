use crate::slack::client::SlackClient;
use crate::slack::emoji::{EmojiListParams, EmojiListResponse::Error, EmojiListResponse::Success};
use rocket;
use rocket::{get, Route};
use std::env;

#[get("/emoji-contributor")]
pub async fn emoji_contributor_route() -> &'static str {
    let token = env::var("SLACK_TOKEN").expect("Please set SLACK_TOKEN");
    let slack_client = SlackClient::new(&token);
    let params = Some(EmojiListParams {
        pretty: 1,
        include_categories: false,
    });
    let emoji_list_response = slack_client.emoji().list(params).await;
    match emoji_list_response {
        Ok(emoji_list_response) => {
            let emoji_list = emoji_list_response.emoji;
            let emoji_list_keys = emoji_list.keys();
            println!("{:?}", emoji_list_keys);
            if !emoji_list_response.ok {
                return "Emoji Contributor - encountered error";
            } else {
                return "Emoji Contributor - not implemented yet";
            }
        },
        Err(error) => {
            println!("Encountered error: {:?}", error);
            return "Could not get emoji list";
        },
    }
}

pub fn routes() -> Vec<Route> {
    routes![emoji_contributor_route]
}
