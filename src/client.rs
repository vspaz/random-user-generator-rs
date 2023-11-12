use crate::config::{get_config, Config};
use reqwest;
use reqwest::blocking::Response;
use std::time::Duration;

pub struct ApiClient {
    api_client: reqwest::blocking::Client,
    config: Config,
}

impl ApiClient {
    pub fn new(client: Option<reqwest::blocking::Client>) -> ApiClient {
        let config = get_config();
        let http_client = client.unwrap_or(
            reqwest::blocking::Client::builder()
                .connect_timeout(config.timeout.connect.to_owned())
                .timeout(config.timeout.general.to_owned())
                .user_agent(&config.user_agent.to_owned())
                .build()
                .unwrap(),
        );
        return ApiClient {
            api_client: http_client,
            config,
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
            .get(self.config.endpoint.as_str().to_owned())
            .send()
            .unwrap();
        resp
    }
}
