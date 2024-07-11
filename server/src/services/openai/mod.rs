mod config;

use async_openai::{config::OpenAIConfig, Client};
use config::Config;
use log::{error, warn};
use crate::utils::configs::get_key;

pub type OaClient = Client<OpenAIConfig>;

pub fn new_oa_client() -> Result<OaClient, Box<dyn std::error::Error>> {
    let mut config = Config::default();
    
    match get_key() {
        Ok(Some(key)) => {
            config.set_key(&key);
        },
        Ok(None) => {
            warn!("API key not found in config file");
        },
        Err(e) => {
            error!("Error reading config file: {}", e);
        }
    }

    if config.is_key_set() {
        let openai_config = OpenAIConfig::new().with_api_key(config.key().unwrap());
        Ok(Client::with_config(openai_config))
    } else {
        Err("No OpenAI API key available".into())
    }
}
