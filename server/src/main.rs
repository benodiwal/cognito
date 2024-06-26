use log::info;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::setup();  
    info!("Hi! My name is Server");
    Ok(())
}
