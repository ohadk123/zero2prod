use config::{self, Config, ConfigError, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DataBaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DataBaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    Config::builder()
        .add_source(File::with_name("configuration"))
        .build()?
        .try_deserialize()
}

impl DataBaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
            )
    }
}
