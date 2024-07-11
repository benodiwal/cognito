use std::fs::read_to_string;
use crate::{args::Args, client::ApiClient, clipboard::{copy, prompt_copy_to_clipboard}};
use log::error;
use serde_json::json;

fn remove_enclosing_backticks(s: &str) -> String {
    if s.len() >= 2 && s.starts_with('`') && s.ends_with('`') {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

pub async fn handle_prompt(client: &ApiClient, prompt: &str) {
    let json_data = json!({
        "prompt": prompt,
    });

    let response = client.post("/process_prompt", &json_data).await;
    match response {
        Ok(mut res) => {
            res = remove_enclosing_backticks(res.as_ref());
            if prompt_copy_to_clipboard(res.as_ref()).unwrap() {
                copy(res.as_ref()).unwrap();
            }
        },
        Err(err) => {
            error!("Error in processing prompt: {}", err);
        }
    }
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
