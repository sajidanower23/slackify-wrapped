use rocket::get;
use rocket::Route;
use std::env;
use crate::slack::slack::SlackClient;
use rocket;

#[get("/emoji-contributor")]
pub async fn emoji_contributor_route() -> &'static str {
    let token = env::var("SLACK_TOKEN").expect("Please set SLACK_TOKEN");
    let slack_client = SlackClient::new(&token);
    let emoji_list_response = slack_client.get_emoji_list().await.unwrap();
    let emoji_list = emoji_list_response.emoji;
    let emoji_list_keys = emoji_list.keys();
    print!("{:?}", emoji_list_keys);
    return "Emoji Contributor - not implemented yet"
}

pub fn routes() -> Vec<Route> {
    routes![emoji_contributor_route]
}
