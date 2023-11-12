use crate::client::ApiClient;
use crate::userdata::Users;

pub fn generate() {
    let api_client = ApiClient::new(None);
    let resp = api_client.fetch_random_user_info("foobar", 1, false, false);
    let payload: Users = resp.json().unwrap();
    println!("{}", payload.results[0].name.first);
}
