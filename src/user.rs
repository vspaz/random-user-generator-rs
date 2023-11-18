use crate::client::ApiClient;
use crate::userdata::User;
use reqwest::Error;

pub struct Data {
    seed: String,
    user_count: u64,
    latin_only: bool,
    name_only: bool,
    api_client: ApiClient,
}

impl Data {
    pub fn new(seed: &str, user_count: u64, latin_only: bool, name_only: bool) -> Self {
        Data {
            seed: seed.to_string(),
            user_count,
            name_only,
            latin_only,
            api_client: ApiClient::new(None),
        }
    }

    pub fn generate(&self) -> Result<Vec<User>, Error> {
        self.api_client.fetch_random_user_info(
            &self.seed,
            self.user_count,
            self.latin_only,
            self.name_only,
        )
    }
}
