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
use crate::models::{TranslationEngine, TranslationRequest, TranslationResponse};
use crate::state::AppState;
use crate::translation::engines::google::translate_google;
use crate::translation::engines::llm::translate_llm;
use std::sync::Arc;
use tauri::State;

pub async fn translate(
    request: TranslationRequest,
    state: State<'_, Arc<AppState>>,
) -> Result<TranslationResponse, String> {
    let client = state.http.read().await.clone();

    match request.engine {
        TranslationEngine::Llm => {
            let llm_settings = state.llm.read().await.clone();
            let res = translate_llm(
                &client,
                &llm_settings,
                &request.text,
                &request.source_lang,
                &request.target_lang,
            )
            .await?;
            Ok(res)
        }
        TranslationEngine::Google => translate_google(
            &client,
            &request.text,
            &request.source_lang,
            &request.target_lang,
        )
        .await
        .map_err(|e| e.to_string()),
    }
}
