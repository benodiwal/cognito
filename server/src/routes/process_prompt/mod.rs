use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PromptRequest {
    prompt: String,
}

#[post("/process_prompt")]
pub async fn process_prompt(req: web::Json<PromptRequest>) -> impl Responder {
    let response = format!("Received prompt: {}", req.prompt);
    HttpResponse::Ok().body(response)
}
