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
    let mut api_client = ApiClient::new(None);
    api_client.config.endpoint = server.url("/api").to_string();
    let resp = api_client.fetch_random_user_info("foobar", 1, true, true);
    let user = &resp.unwrap()[0];

    assert_eq!("Lewis", user.name.first);
    assert_eq!("Schmidt", user.name.last);
    assert_eq!("Mr", user.name.title);
}


#[test]
fn test_fetch_random_user_info_full_ok() {
    use httptest::{matchers::*, responders::*, Expectation, Server};
    let server = Server::run();
    let response_body = r#"
    {"results":[{"gender":"male","name":{"title":"Mr","first":"Macit","last":"Ayverdi"},
    "location":{"street":{"number":5400,"name":"Şehitler Cd"},
    "city":"Çanakkale","state":"Adana","country":"Turkey",
    "postcode":67098,"coordinates":{"latitude":"66.9463",
    "longitude":"-159.8609"},"timezone":{"offset":"-12:00",
    "description":"Eniwetok, Kwajalein"}},
    "email":"macit.ayverdi@example.com",
    "login":{"uuid":"a208530a-e829-4ec6-9dbc-a5c08b4b2f28",
    "username":"redrabbit675","password":"spawn",
    "salt":"6GlM6Vyt","md5":"4020ceb4bac4a626132b621993c0513e",
    "sha1":"5ed48a18fbd294bfce26460af6f763431e58a206","sha256":
    "85026b9af10713b1a324a230ab21042593dbcd3d0d5da30f478a7f80fbdb6269"},
    "dob":{"date":"1965-02-15T17:40:07.930Z","age":58},
    "registered":{"date":"2018-01-01T14:59:15.851Z","age":5},
    "phone":"(572)-719-1512","cell":"(575)-559-8394","id":{"name":"","value":null},
    "picture":{"large":"https://randomuser.me/api/portraits/men/4.jpg",
    "medium":"https://randomuser.me/api/portraits/med/men/4.jpg",
    "thumbnail":"https://randomuser.me/api/portraits/thumb/men/4.jpg"},
    "nat":"TR"}],"info":{"seed":"6268613e61d773f3","results":1,"page":1,"version":"1.4"}}
    "#;
    server.expect(
        Expectation::matching(request::method_path("GET", "/api"))
            .respond_with(status_code(200).body(response_body)),
    );
    let mut api_client = ApiClient::new(None);
    api_client.config.endpoint = server.url("/api").to_string();
    let resp = api_client.fetch_random_user_info("foobar", 1, true, true);
    let user = &resp.unwrap()[0];

    if let Some(location) = user.location.as_ref() {
        if let Some(city) = location.city.as_ref() {
            assert_eq!("Çanakkale", city)
        }
    }
    if let Some(email) = user.email.as_ref() {
        assert_eq!("macit.ayverdi@example.com", email);
    }
}