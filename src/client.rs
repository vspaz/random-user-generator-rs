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
