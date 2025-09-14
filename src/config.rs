use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub couchbase: CouchbaseConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CouchbaseConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub timeout_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    pub enabled: bool,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        let mut settings = config::Config::builder()
            .add_source(config::Environment::default())
            .set_default("server.port", 8080)?
            .set_default("server.host", "0.0.0.0")?
            .set_default("couchbase.host", "http://localhost:8091")?
            .set_default("couchbase.username", "Administrator")?
            .set_default("couchbase.password", "password")?
            .set_default("couchbase.timeout_seconds", 30)?
            .set_default("auth.enabled", true)?
            .set_default("auth.username", "admin")?
            .set_default("auth.password", "admin")?;

        // Override with environment variables
        if let Ok(port) = env::var("PORT") {
            if let Ok(port) = port.parse::<u16>() {
                settings = settings.set_override("server.port", port)?;
            }
        }

        if let Ok(host) = env::var("COUCHBASE_HOST") {
            settings = settings.set_override("couchbase.host", host)?;
        }

        if let Ok(username) = env::var("COUCHBASE_USERNAME") {
            settings = settings.set_override("couchbase.username", username)?;
        }

        if let Ok(password) = env::var("COUCHBASE_PASSWORD") {
            settings = settings.set_override("couchbase.password", password)?;
        }

        if let Ok(timeout) = env::var("COUCHBASE_TIMEOUT_SECONDS") {
            if let Ok(timeout) = timeout.parse::<u64>() {
                settings = settings.set_override("couchbase.timeout_seconds", timeout)?;
            }
        }

        if let Ok(enabled) = env::var("AUTH_ENABLED") {
            if let Ok(enabled) = enabled.parse::<bool>() {
                settings = settings.set_override("auth.enabled", enabled)?;
            }
        }

        if let Ok(username) = env::var("AUTH_USERNAME") {
            settings = settings.set_override("auth.username", username)?;
        }

        if let Ok(password) = env::var("AUTH_PASSWORD") {
            settings = settings.set_override("auth.password", password)?;
        }

        settings.build()?.try_deserialize()
    }
}
