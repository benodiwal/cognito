use std::{error::Error, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::setup();

    let settings = server::configurations::get_configurations()?;
    let listener = TcpListener::bind(format!("{}:{}", settings.get_host(), settings.get_application_port()))?;
    server::run(listener).await?.await?;

    Ok(())
}
