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

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Deserialize, Type)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct KeyStatus {
    pub profile_id: String,
    pub is_saved: bool,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", content = "message")]
pub enum KeyStoreError {
    MissingKeyringDaemon(String),
    Unknown(String),
}
// Превращаем любую ошибку keyring::Error в наш KeyStoreError
impl From<keyring::Error> for KeyStoreError {
    fn from(err: keyring::Error) -> Self {
        match err {
            keyring::Error::NoDefaultStore => KeyStoreError::MissingKeyringDaemon(
                "Secure credential storage is unavailable. Please install and start a Secret Service daemon (such as gnome-keyring or kwallet) to safely save your API keys".to_string(),
            ),
            e => KeyStoreError::Unknown(e.to_string()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TranslationRequest {
    pub engine: TranslationEngine,
    pub text: String,
    pub source_lang: String,
    pub target_lang: String,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TranslationResponse {
    pub translation: String,
    pub detected_source_lang: Option<String>,
    pub source_correction: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[serde(rename_all = "lowercase")]
pub enum TranslationEngine {
    Llm,
    Google,
}
