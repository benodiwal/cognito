use std::env;
use log::error;

pub fn load() {
    match dotenv::dotenv() {
        Ok(_) => (),
        Err(err) => {
            error!("Error loading environment varaibles: {}", err);
            std::process::exit(1);       
        }
    }
}

pub fn read_env(key: &str) -> String {
    match env::var(key) {
        Ok(value) => value,
        Err(err) => {
            error!("Error reading environment varaibles {}: {}", key, err);
            std::process::exit(1);
        }
    }
}
