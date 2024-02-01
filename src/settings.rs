use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub server: ServerSettings,
    pub db: DbSettings,
    pub kv: KVSettings,
}

#[derive(Deserialize, Debug)]
pub struct ServerSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub concurrency_limit: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub client_timeout_in_millis: u64,
}

#[derive(Deserialize, Debug)]
pub struct DbSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub pool_size: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timeout_in_millis: u64,
}

#[derive(Deserialize, Debug)]
pub struct KVSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub pool_size: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timeout_in_millis: u64,
}

impl ServerSettings {
    pub fn address(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    pub fn address_wo_protocol(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl DbSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .ssl_mode(PgSslMode::Disable)
    }
    pub fn with_db(&self) -> PgConnectOptions {
        let options = self.without_db()
            .database(&self.database_name)
            .log_statements(log::LevelFilter::Trace);
        options
    }
}

impl KVSettings {
    pub fn get_redis_url(&self) -> String {
        format!("redis://{}:{}/", self.host, self.port)
    }
}

pub fn get_config(location: &str) -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to get the current directory");

    let settings = config::Config::builder()
        .add_source(config::File::from(
            base_path.join(location),
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}