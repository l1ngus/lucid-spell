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
use crate::state::AppState;
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
