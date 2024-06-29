use std::{fs::File, io::{Result, Write}, path::PathBuf};
use dirs::home_dir;
use log::{error, info};
use crate::args::Args;

fn get_file_path() -> PathBuf {
    if let Some(home) = home_dir() {
        home.join(".cognito_config")
    } else {
        error!("Couldn't find home directory");
        std::process::exit(1);
    }
}

fn create_config_file() -> Result<File> {
    let file_path = get_file_path();
    let file = File::create(file_path)?;
    Ok(file)
}

fn set_key(key: &str) {

    if key.is_empty() {
        error!("Key can't be empty");
        std::process::exit(1);
    }

    let mut config_file = create_config_file().unwrap();
    let to_write = format!("OPENAI_KEY={}", key);
    match config_file.write_all(to_write.as_bytes()) {
      Ok(()) => {
        info!("OPENAI_KEY set to {} successfully ✅", key);
        std::process::exit(0);
      },
      Err(err) => {
        error!("Error writing Config file: {}", err);
        std::process::exit(1);
      }  
    };
}

pub fn set_config(args: &Args) {
    if let Some(key) = &args.config {
        set_key(key);
    }
}

pub fn check_config() -> bool {
    let file_path = get_file_path();
    match std::fs::metadata(file_path) {
        Ok(_) => {
            info!("Checking config file ....., File exists 🎉");
            true
        },
        Err(_) => {
            error!("Config file does not exist. Please set config using --config command");
            std::process::exit(1);
        }
    }
}
