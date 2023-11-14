use crate::config::{get_config, Config};
use crate::userdata::{User, Users};
use reqwest::Error;
use simple_log::{debug, LogConfigBuilder};

pub struct ApiClient {
    api_client: reqwest::blocking::Client,
    config: Config,
}

impl ApiClient {
    pub fn new(client: Option<reqwest::blocking::Client>) -> ApiClient {
        let config = get_config();
        let log_config = LogConfigBuilder::builder()
            .time_format(&config.logging.time_format)
            .level(&config.logging.level)
            .output_console()
            .build();
        simple_log::new(log_config).expect("failed to configure logger");
        let http_client = client.unwrap_or(
            reqwest::blocking::Client::builder()
                .connect_timeout(config.timeout.connect.to_owned())
                .timeout(config.timeout.general.to_owned())
                .user_agent(config.user_agent.to_owned())
                .build()
                .unwrap(),
        );
        ApiClient {
            api_client: http_client,
            config,
        }
    }

    pub fn fetch_random_user_info(
        &self,
        seed: &str,
        user_count: u64,
        latin_only: bool,
        name_only: bool,
    ) -> Result<Vec<User>, Error> {
        let result_count = user_count.to_string();
        let mut query_params = vec![("seed", seed), ("results", result_count.as_str())];

        if latin_only {
            query_params.push(("nat", self.config.default_countries.as_str()))
        }

        if name_only {
            query_params.push(("inc", "name"))
        }
        match self
            .api_client
            .get(self.config.endpoint.as_str().to_owned())
            .query(&query_params)
            .send()
        {
            Ok(resp) => {
                debug!("status received: {}", &resp.status().as_str());
                let body: Users = resp.json()?;
                Ok(body.results)
            }
            Err(e) => Err(e),
        }
    }
}

#[test]
fn test_fetch_random_user_info_name_ok() {
    use httptest::{matchers::*, responders::*, Expectation, Server};
    let server = Server::run();
    let response_body = r#"
    {"results":[{"name":{"title":"Mr","first":"Lewis","last":"Schmidt"}}],
    "info":{"seed":"2b834efd58cd6a19","results":1,"page":1,"version":"1.4"}}
    "#;
    server.expect(
        Expectation::matching(request::method_path("GET", "/api"))
            .respond_with(status_code(200).body(response_body)),
    );
    let mut config = get_config();
    config.endpoint = server.url("/api").to_string();
    let api_client = ApiClient {
        api_client: reqwest::blocking::Client::builder()
            .connect_timeout(config.timeout.connect.to_owned())
            .timeout(config.timeout.general.to_owned())
            .user_agent(config.user_agent.to_owned())
            .build()
            .unwrap(),
        config,
    };
    let resp = api_client.fetch_random_user_info("foobar", 1, true, true);
    assert_eq!("Lewis", resp.unwrap()[0].name.first)
}
