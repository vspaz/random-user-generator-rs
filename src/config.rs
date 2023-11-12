use std::time::Duration;

pub struct Timeout {
    connect: Duration,
    general: Duration,
}

pub struct Config {
    endpoint: String,
    user_agent: String,
    timeout: Timeout,
}

pub fn get_config() -> Config {
    Config {
        endpoint: "https://randomuser.me/api".to_string(),
        user_agent: "randomuser".to_string(),
        timeout: Timeout{
            connect: Duration::from_secs(10),
            general: Duration::from_secs(10),
        }
    }
}
