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

use crate::dto::ChatMessage;
use async_openai::types::chat::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
};
pub fn parse_chat_messages(
    messages: Vec<ChatMessage>,
) -> Result<Vec<ChatCompletionRequestMessage>, String> {
    messages
        .into_iter()
        .map(|msg| match msg.role.as_str() {
            "user" => ChatCompletionRequestUserMessageArgs::default()
                .content(msg.content)
                .build()
                .map_err(|e| e.to_string())
                .map(|m| m.into()),

            "assistant" => ChatCompletionRequestAssistantMessageArgs::default()
                .content(msg.content)
                .build()
                .map_err(|e| e.to_string())
                .map(|m| m.into()),

            "system" => ChatCompletionRequestSystemMessageArgs::default()
                .content(msg.content)
                .build()
                .map_err(|e| e.to_string())
                .map(|m| m.into()),

            _ => Err(format!("Unknown role: {}", msg.role)),
        })
        .collect()
}
