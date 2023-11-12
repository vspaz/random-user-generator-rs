use crate::client::ApiClient;
use crate::userdata::{User, Users};

pub struct RandomUserData {
    seed: String,
    user_count: u64,
    latin_only: bool,
    name_only: bool,
    api_client: ApiClient,
}

impl RandomUserData {
    pub fn new(seed: &str, user_count: u64, latin_only: bool, name_only: bool) -> RandomUserData {
        return RandomUserData {
            seed: String::from(seed),
            user_count,
            name_only,
            latin_only,
            api_client: ApiClient::new(None),
        };
    }

    pub fn generate(&self) -> Vec<User> {
        let resp = self.api_client.fetch_random_user_info(
            &self.seed,
            self.user_count,
            self.latin_only,
            self.name_only,
        );
        let payload: Users = resp.json().unwrap();
        payload.results
    }
}
