use std::{error::Error, net::TcpListener};

mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::setup();

    let listener = TcpListener::bind("127.0.0.1:8082")?;
    server::run(listener).await?.await?;

    Ok(())
}
