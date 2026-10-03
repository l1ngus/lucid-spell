/*
 * Copyright (C) 2026 l1ngus
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::logic::parse_chat_messages;
use crate::models::ChatMessage;
use async_openai::{config::OpenAIConfig, types::chat::CreateChatCompletionRequestArgs, Client};
use serde_json::Value;

pub async fn ask_llm(
    http_client: &reqwest::Client,
    api_url: impl Into<String>,
    api_key: impl Into<String>,
    messages: Vec<ChatMessage>,
    model: impl Into<String>,
    temperature: f32,
) -> Result<String, String> {
    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_api_base(api_url);

    let client = Client::with_config(config).with_http_client(http_client.clone());

    let api_messages =
        parse_chat_messages(messages).map_err(|e| format!("Parsing chat messages error: {}", e))?;

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .max_tokens(4096u16)
        .temperature(temperature)
        .messages(api_messages)
        .stream(false)
        .build()
        .map_err(|e| format!("Failed to build request: {}", e))?;

    let resp_json: Value = client
        .chat()
        .create_byot(request)
        .await
        .map_err(|e| format!("API request failed: {}", e))?;

    // 1. Ошибка провайдера
    if let Some(api_error) = resp_json.get("error") {
        return Err(format!("LLM Provider Error: {}", api_error));
    }

    // 2. Первый choice
    let choice = resp_json
        .get("choices")
        .and_then(|c| c.as_array())
        .and_then(|arr| arr.get(0))
        .ok_or("Invalid API response: 'choices' array is missing or empty")?;

    // 3. Обрезка по max_tokens
    if let Some(finish_reason) = choice.get("finish_reason").and_then(|v| v.as_str()) {
        if finish_reason == "length" {
            return Err(
                "Token limit exceeded (max_tokens). Response was truncated before completion."
                    .to_string(),
            );
        }
    }

    // 4. Контент
    let content = choice
        .get("message")
        .and_then(|m| m.get("content"))
        .and_then(|v| v.as_str())
        .ok_or("Invalid API response: 'message.content' is missing or not a string")?
        .to_string();

    Ok(content)
}
