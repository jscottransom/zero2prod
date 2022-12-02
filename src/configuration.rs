#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

// All fields within the Setting struct need to be deserializable in order for the entire struct to be
// as well.
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize the config reader
    let mut settings = config::Config::default();

    // Add the config valuesfrom a file named `configuration`.
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()
}
