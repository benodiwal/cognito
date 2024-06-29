mod prompt;
mod args;
mod repl;
mod client;
mod config;
mod env;
mod constants;

use client::{new_api_client, ApiClient};
use config::{check_config, set_config};
use log::{error, info};
use prompt::handle_prompt;

async fn check_server(client: &ApiClient) {
    let response = client.get("/health").await; 
    match response {
        Ok(_) => {
            info!("Server running ✅");
        },
        Err(_) => {
            error!("Error Server not running ❌");
            std::process::exit(1);
        }
    }   
}

pub async fn init() {
    logger::setup();
    env::load();

    let api_client = new_api_client();
    check_server(&api_client).await;

    let _args = args::parse();
    set_config(&_args);

    if check_config() {
        let pr = prompt::read(&_args);
        handle_prompt(&api_client, pr.as_ref()).await;

        if _args.repl {
            repl::start();
        }
    }
}
