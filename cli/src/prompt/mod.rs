use std::fs::read_to_string;
use crate::args::Args;
use log::error;

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
