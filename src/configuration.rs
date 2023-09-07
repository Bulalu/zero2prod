use secrecy::{Secret, ExposeSecret};


#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,

}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username,
                self.password.expose_secret(),
                self.host,
                self.port,
                self.database_name,

            )
        )
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
           Secret::new(
                format!(
                    "postgres://{}:{}@{}:{}",
                    self.username,
                    self.password.expose_secret(),
                    self.host,
                    self.port,

                )
            )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Load Variable from .env file
    dotenv::dotenv().ok();
    // Initialize our configuration reader
    let mut settings = config::Config::default();

    // // Add in settings from environment variables (with a prefix of APP and '__' as separator)
    // // E.g., `APP_DATABASE__PORT=5432 would set `Settings.database.port`
    // settings.merge(config::Environment::with_prefix("APP").separator("__"))?;
    //

    // Add configuration values from a file named `configuration`.
    // It will look for any top-level file with an extension
    // that `config` knows how to parse: yaml, json, etc.
    settings.merge(config::File::with_name("configuration"))?;

    settings.try_into()
}

