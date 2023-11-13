use std::time::Duration;

pub struct Logging {
    pub level: String,
    pub time_format: String,
}

pub struct Timeout {
    pub connect: Duration,
    pub general: Duration,
}

pub struct Config {
    pub endpoint: String,
    pub user_agent: String,
    pub default_countries: String,
    pub timeout: Timeout,
    pub logging: Logging,
}

pub fn get_config() -> Config {
    Config {
        endpoint: "https://randomuser.me/api".to_string(),
        user_agent: "randomuser".to_string(),
        default_countries: "us,gb,au,ca,ie".to_string(),
        timeout: Timeout {
            connect: Duration::from_secs(10),
            general: Duration::from_secs(10),
        },
        logging: Logging {
            level: "info".to_string(),
            time_format: "%Y-%m-%d %H:%M:%S.%3f".to_string(),
        },
    }
}
