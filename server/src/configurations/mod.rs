use config::{File, FileFormat};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    server_port: u16,
    host: String,
}

pub fn get_configurations() -> Result<Settings, config::ConfigError> {
    let builder = config::Config::builder()
    .add_source(File::new("configurations", FileFormat::Yaml));

    let config = builder.build()?;
    config.try_deserialize::<Settings>()
}

impl Settings {
    pub fn get_application_port(&self) -> u16 {
        self.server_port
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }
}
