mod config;

use async_openai::{config::OpenAIConfig, Client};
use config::Config;

pub type OaClient = Client<OpenAIConfig>;

pub fn new_oa_client() -> Result<OaClient, Box<dyn std::error::Error>> {
    let config = Config::default();
    todo!()
}
