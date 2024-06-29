use log::error;
use reqwest::{Client, Url, Result as ReqwestResult};
use crate::{constants, env};

pub struct ApiClient {
    client: Client,
    base_url: Url,
}

impl ApiClient {
    fn new(base_url: &str) -> ReqwestResult<Self> {
        let client = Client::new();
        let base_url = Url::parse(base_url).unwrap();
        Ok(ApiClient { client, base_url })
    }

    pub async fn get(&self, endpoint: &str) -> ReqwestResult<String> {
        let url = self.base_url.join(endpoint).unwrap();
        let respone = self.client.get(url).send().await?;
        let body = respone.text().await?;
        Ok(body)
    }

    pub async fn post(&self, endpoint: &str, json_data: &serde_json::Value) -> ReqwestResult<String> {
        let url = self.base_url.join(endpoint).unwrap();
        let body = serde_json::to_string(json_data).unwrap();

        let response = self.client.post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(body)
        .send().await?;

        let body = response.text().await?;
        Ok(body)        
    }
}

pub fn new_api_client() -> ApiClient {
    let base_url = format!("http://localhost:{}", env::read_env(constants::PORT));
    match ApiClient::new(&base_url) {
        Ok(client) => client,
        Err(err) => {
            error!("Error creating api client: {}", err);
            std::process::exit(1);
        }
    }
}
