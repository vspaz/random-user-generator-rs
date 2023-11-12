use std::time::Duration;

pub struct Timeout {
    pub connect: Duration,
    pub general: Duration,
}

pub struct Config {
    pub endpoint: String,
    pub user_agent: String,
    pub timeout: Timeout,
}

pub fn get_config() -> Config {
    Config {
        endpoint: "https://randomuser.me/api".to_string(),
        user_agent: "randomuser".to_string(),
        timeout: Timeout {
            connect: Duration::from_secs(10),
            general: Duration::from_secs(10),
        },
    }
}
