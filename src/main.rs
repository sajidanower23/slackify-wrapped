#[macro_use] extern crate rocket;

#[get("/health")]
fn health() -> &'static str {
    "Health!"
}

#[get("/version")]
fn version() -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    format!("Version: {}!", VERSION)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![version, health])
}
