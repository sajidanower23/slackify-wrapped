#[cfg(test)]
mod test {
    use crate::rocket;
    use rocket::local::blocking::Client;

    #[test]
    fn health() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/health").dispatch();

        use rocket::http::Status;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Health!".into()));
    }

    #[test]
    fn version() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/version").dispatch();

        use rocket::http::Status;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Version: 0.1.0!".into()));
    }
}
