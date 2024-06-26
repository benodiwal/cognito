pub mod routes;

use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    
    let server = HttpServer::new(|| {
        App::new()
        .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
