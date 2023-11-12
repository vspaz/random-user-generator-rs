use reqwest;
use reqwest::blocking::Response;

pub struct ApiClient {
    api_client: reqwest::blocking::Client,
}

impl ApiClient {
    pub fn new(client: Option<reqwest::blocking::Client>) -> ApiClient {
        let http_client = client.unwrap_or(reqwest::blocking::Client::new());
        return ApiClient {
            api_client: http_client,
        };
    }

    pub fn fetch_random_user_info(
        &self,
        seed: &str,
        user_count: u64,
        latin_only: bool,
        name_only: bool,
    ) -> Response {
        let resp = self
            .api_client
            .get("https://randomuser.me/api")
            .send()
            .unwrap();
        resp
    }
}