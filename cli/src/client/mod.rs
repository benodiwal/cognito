use log::error;
use reqwest::{Client, Url, Result};

use crate::{constants, env};

pub struct ApiClient {
    pub client: Client,
    pub base_url: Url,
}

impl ApiClient {
    fn new(base_url: &str) -> Result<Self> {
        let client = Client::new();
        let base_url = Url::parse(base_url).unwrap();
        Ok(ApiClient { client, base_url })
    }

    async fn get(&self, endpoint: &str) -> Result<String> {
        let url = self.base_url.join(endpoint).unwrap();
        let respone = self.client.get(url).send().await?;
        let body = respone.text().await?;
        Ok(body)
    }
}

pub fn create_api_client() -> ApiClient {
    let base_url = format!("http://localhost:{}", env::read_env(constants::PORT));
    match ApiClient::new(&base_url) {
        Ok(client) => client,
        Err(err) => {
            error!("Error creating api client: {}", err);
            std::process::exit(1);
        }
    }
}
