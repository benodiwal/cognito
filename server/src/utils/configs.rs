use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};
use log::error;
use dirs::home_dir;

use super::constants;

fn get_file_path() -> PathBuf {
    if let Some(home) = home_dir() {
        home.join(".cognito_config")
    } else {
        error!("Couldn't find home directory");
        std::process::exit(1);
    }
}

pub fn get_key() -> Result<Option<String>, std::io::Error> {
    let path = get_file_path();
    let key = constants::KEY;

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 && parts[0].trim() == key {
            return Ok(Some(parts[1].trim().to_string()));
        }
    }

    Ok(None)
}
