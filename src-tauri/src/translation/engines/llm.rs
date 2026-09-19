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
use crate::llm::client::ask_llm;
use crate::llm::parser::clean_llm_output;
use crate::llm::prompts::{translation_prompt, TranslationPromptOutput};
use crate::logic::keychain::get_key;
use crate::models::{ChatMessage, TranslationResponse};
use crate::state::LlmSettings;

pub async fn translate_llm(
    client: &reqwest::Client,
    llm_settings: &LlmSettings,
    text: &str,
    source_lang: &str,
    target_lang: &str,
) -> Result<TranslationResponse, String> {
    let prompt = translation_prompt(text, source_lang, target_lang).map_err(|e| e.to_string())?;

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

    let output: TranslationPromptOutput =
        serde_json::from_str(&clean_answer).map_err(|e| e.to_string())?;

    let translation_response: TranslationResponse = TranslationResponse {
        translation: output.translation,
        detected_source_lang: None,
        source_correction: Some(output.source_correction),
    };

    Ok(translation_response)
}
