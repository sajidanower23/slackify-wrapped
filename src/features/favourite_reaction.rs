use crate::slack::client::SlackClient;
use rocket;
use rocket::{get, Route};
use std::env;

#[get("/favourite-reaction")]
pub async fn favourite_reaction() -> &'static str {
    let token = env::var("SLACK_TOKEN").expect("Please set SLACK_TOKEN");
    let _slack_client = SlackClient::new(&token);
    // This requires reactions.list to be implemented first
    return "Not implemented yet";
}

pub fn routes() -> Vec<Route> {
    routes![favourite_reaction]
}
