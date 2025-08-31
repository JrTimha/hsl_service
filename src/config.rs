use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct HLSConfig {
    pub hsl_port: u16,
    pub hsl_url: String,
    pub log_level: String,
    pub object_db_config: ObjectDbConfig
}

#[derive(Deserialize, Debug, Clone)]
pub struct ObjectDbConfig {
    pub db_user: String,
    pub db_url: String,
    pub db_password: String,
    pub bucket_name: String
}


//examples: https://github.com/rust-cli/config-rs/blob/main/examples/hierarchical-env/settings.rs
impl HLSConfig {

    pub fn new(mode: &str) -> Result<Self, ConfigError> {
        //layering the different environment variables, default values first, overwritten by config files and env-vars
        let config = Config::builder()
            .add_source(File::with_name("default.config.toml"))
            .add_source(File::with_name(&format!("{mode}.config.toml")).required(false))
            .add_source(Environment::default().separator("__"))
            .build()?;
        config.try_deserialize()
    }
}