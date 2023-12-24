use rocket::get;
use rocket::Route;
use reqwest;
use rocket;

const TOKEN: &str = "";

#[get("/emoji-contributor")]
pub async fn emoji_contributor_route() -> &'static str {
    let token = TOKEN;
    let client = reqwest::Client::new();
    let url = "https://slack.com/api/emoji.list";
    let _response =  match client.get(url).header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await {
                        Ok(resp) => {
                            println!("Response: {:?}", resp);
                            resp
                        },
                        Err(e) => {
                            println!("Error: {}", e);
                            return "Error";
                        }
                    };
    return "Emoji Contributor"
}

pub fn routes() -> Vec<Route> {
    routes![emoji_contributor_route]
}
