use actix_web::{post, web, HttpResponse, Responder};
use async_openai::types::ChatCompletionRequestMessage;
use serde::Deserialize;

use crate::services::openai::{chats::chat, get_config, messages::{default_messages, user_message}, new_oa_client};

#[derive(Deserialize)]
pub struct PromptRequest {
    prompt: String,
}

#[post("/process_prompt")]
pub async fn process_prompt(req: web::Json<PromptRequest>) -> impl Responder {

    let config = get_config().unwrap();
    let oac = new_oa_client(&config).unwrap();

    let mut messages = default_messages();
    let user_message = ChatCompletionRequestMessage::User(user_message(&req.prompt));
    messages.push(user_message);

    let response = chat(&oac, &config, messages).await.unwrap();

    HttpResponse::Ok().body(response)
}
