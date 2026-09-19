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

use crate::logic::keychain;
// use crate::models::ChatMessage;
use crate::state::AppState;
// use async_openai::{config::OpenAIConfig, types::chat::CreateChatCompletionRequestArgs, Client};
// use reqwest::{ClientBuilder, Proxy};
// use serde_json::Value;
use specta;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn set_llm_config(
    profile_id: String,
    api_url: String,
    model: String,
    temperature: f32,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    keychain::get_key(&profile_id).map_err(|e| e.to_string())?;
    let mut llm_config = state.llm.write().await;
    llm_config.profile_id = profile_id;
    llm_config.api_url = api_url;
    llm_config.model = model;
    llm_config.temperature = temperature;

    Ok(())
}

// #[tauri::command]
// #[specta::specta]
// pub async fn set_llm_config(
//     profile_id: String,
//     api_url: String,
//     proxy_url: Option<String>,
//     state: State<'_, Arc<AppState>>,
// ) -> Result<(), String> {
//     let mut client_builder = ClientBuilder::new();
//
//     let api_key = keychain::get_key(&profile_id).map_err(|e| e.to_string())?;
//
//     if let Some(proxy) = proxy_url {
//         let proxy = Proxy::all(&proxy).map_err(|e| format!("Invalid proxy URL: {}", e))?;
//         client_builder = client_builder.proxy(proxy);
//     }
//
//     let http_client = client_builder.build().map_err(|e| e.to_string())?;
//     let config = OpenAIConfig::new()
//         .with_api_key(api_key)
//         .with_api_base(api_url);
//     let client = Client::with_config(config).with_http_client(http_client);
//
//     let mut client_guard = state.openai_client.lock().await;
//     *client_guard = Some(client);
//
//     Ok(())
// }

// #[tauri::command]
// #[specta::specta]
// pub async fn ask_llm(
//     messages: Vec<ChatMessage>,
//     model: String,
//     temperature: f32,
//     state: tauri::State<'_, Arc<AppState>>,
// ) -> Result<String, String> {
//     let client_guard = state.openai_client.lock().await;
//     let client = client_guard.as_ref().ok_or("API key not set")?;
//
//     let api_messages = match parse_chat_messages(messages) {
//         Ok(msgs) => msgs,
//         Err(e) => return Err(format!("Parsing chat messages error: {}", e)),
//     };
//
//     let request = CreateChatCompletionRequestArgs::default()
//         .model(model)
//         .max_tokens(4096u16)
//         .temperature(temperature)
//         .messages(api_messages)
//         .stream(false)
//         .build()
//         .map_err(|e| format!("Failed to build request: {}", e))?;
//
//     let resp_json: Value = client
//         .chat()
//         .create_byot(request)
//         .await
//         .map_err(|e| format!("API request failed: {}", e))?;
//
//     // 1. Check if the LLM provider returned an error payload
//     if let Some(api_error) = resp_json.get("error") {
//         return Err(format!("LLM Provider Error: {}", api_error));
//     }
//
//     // 2. Retrieve the first choice object
//     let choice = resp_json
//         .get("choices")
//         .and_then(|c| c.as_array())
//         .and_then(|arr| arr.get(0))
//         .ok_or("Invalid API response: 'choices' array is missing or empty")?;
//
//     // 3. Handle truncation due to max_tokens limits
//     if let Some(finish_reason) = choice.get("finish_reason").and_then(|v| v.as_str()) {
//         if finish_reason == "length" {
//             return Err(
//                 "Token limit exceeded (max_tokens). Response was truncated before completion."
//                     .to_string(),
//             );
//         }
//     }
//
//     // 4. Extract content string safely
//     let content = choice
//         .get("message")
//         .and_then(|m| m.get("content"))
//         .and_then(|v| v.as_str())
//         .ok_or("Invalid API response: 'message.content' is missing or not a string")?
//         .to_string();
//
//     Ok(content)
// }
