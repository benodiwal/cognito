pub mod config;
pub mod chats;
pub mod messages;

use async_openai::{config::OpenAIConfig, Client};
use config::Config;
use log::{error, warn};
use crate::utils::configs::get_key;
use std::error::Error;

pub type OaClient = Client<OpenAIConfig>;

pub fn get_config() -> Result<Config, Box<dyn Error>> {
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
            return Err(Box::new(e));
        }
    }

    if config.is_key_set() {
        Ok(config)
    } else {
        Err("No OpenAI API key available".into())
    }
}

pub fn new_oa_client(config: &Config) -> Result<OaClient, Box<dyn Error>> {
    if config.is_key_set() {
        let openai_config = OpenAIConfig::new().with_api_key(config.key().unwrap());
        Ok(Client::with_config(openai_config))
    } else {
        Err("No OpenAI API key available".into())
    }
}
