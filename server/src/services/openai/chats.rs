use async_openai::types::ChatCompletionRequestMessage;
use async_openai::types::CreateChatCompletionRequestArgs;
use super::Config;
use super::OaClient;

pub async fn chat(oac: &OaClient, config: &Config, messages: Vec<ChatCompletionRequestMessage>) -> Result<String, Box<dyn std::error::Error>> {

    let request = CreateChatCompletionRequestArgs::default()
    .model(config.model())
    .messages(messages)
    .temperature(0.8)
    .max_tokens(64_u32)
    .frequency_penalty(0.5)
    .presence_penalty(0.0)
    .build()?;

    let response = oac.chat().create(request).await?;

    if let Some(choice) = response.choices.first() {
        Ok(choice.message.content.as_ref().unwrap().clone())
    } else {
        Err("No response generator".into())
    }

}
