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
use crate::app_state::RemoteLlmSettings;
use crate::dto::{ChatMessage, OtherTranslationEntry, OtherTranslationsResponse};
use crate::infra::keychain::get_key;
use crate::remote_llm::openai_client::ask_llm;
use crate::remote_llm::prompts::other_translations_prompt;
use crate::remote_llm::sanitize::clean_llm_output;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawOtherTranslationsResponse {
    other_translations: Vec<RawOtherTranslationEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RawOtherTranslationEntry {
    Simple(String),
    WithPart { part: String, translation: String },
}

pub async fn get_other_translations_llm(
    client: &reqwest::Client,
    llm_settings: &RemoteLlmSettings,
    source_text: &str,
    translated_text: &str,
    source_lang: &str,
    target_lang: &str,
) -> Result<OtherTranslationsResponse, String> {
    let prompt = other_translations_prompt(source_text, translated_text, source_lang, target_lang)
        .map_err(|e| e.to_string())?;

    let api_key = get_key(&llm_settings.profile_id).map_err(|e| e.to_string())?;

    let messages = vec![ChatMessage {
        role: "user".to_string(),
        content: prompt,
    }];

    let answer = ask_llm(
        client,
        &llm_settings.api_url,
        api_key,
        messages,
        &llm_settings.model,
        llm_settings.temperature,
    )
    .await?;

    let clean_answer = clean_llm_output(answer);

    let raw: RawOtherTranslationsResponse =
        serde_json::from_str(&clean_answer).map_err(|e| e.to_string())?;

    let other_translations = raw
        .other_translations
        .into_iter()
        .map(|entry| match entry {
            RawOtherTranslationEntry::Simple(translation) => OtherTranslationEntry {
                part: None,
                translation,
            },
            RawOtherTranslationEntry::WithPart { part, translation } => OtherTranslationEntry {
                part: Some(part),
                translation,
            },
        })
        .collect();

    Ok(OtherTranslationsResponse { other_translations })
}
