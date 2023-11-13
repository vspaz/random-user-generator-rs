use crate::client::ApiClient;
use crate::userdata::{User, Users};
use reqwest::Error;

pub struct RandomUserData {
    seed: String,
    user_count: u64,
    latin_only: bool,
    name_only: bool,
    api_client: ApiClient,
}

impl RandomUserData {
    pub fn new(seed: &str, user_count: u64, latin_only: bool, name_only: bool) -> RandomUserData {
        RandomUserData {
            seed: seed.to_string(),
            user_count,
            name_only,
            latin_only,
            api_client: ApiClient::new(None),
        }
    }

    pub fn generate(&self) -> Result<Vec<User>, Error> {
        return match self.api_client.fetch_random_user_info(
            &self.seed,
            self.user_count,
            self.latin_only,
            self.name_only,
        ) {
            Ok(resp) => {
                let body: Users = resp.json()?;
                Ok(body.results)
            }
            Err(e) => Err(e),
        };
    }
}
