use std::fs::read_to_string;
use crate::{args::Args, client::new_api_client};
use log::error;

pub async fn handle_prompt() {
    let api_client = new_api_client();
    let res = api_client.get("/health").await.unwrap();
    println!("{}", res);
}

pub fn read(args: &Args) -> String {
    if let Some(file_path) = &args.prompt_file {
        match read_to_string(file_path) {
            Ok(mut prompt) => {
                if matches!(prompt.as_bytes().last(), Some(b'\n')) {
                    prompt.pop();  
                }
                if matches!(prompt.as_bytes().last(), Some(b'\r')) {
                    prompt.pop();
                }
                prompt
            },
            Err(err) => {
                error!("Error reading prompt from {}: {}", file_path, err);
                std::process::exit(1);
            },
        }
    } else if let Some(prompt) = &args.prompt {
        prompt.clone()
    } else if args.repl {
        "".to_string()
    } else {
        error!("No prompt or prompt file was provided. See --help");
        std::process::exit(1);
    }
}
