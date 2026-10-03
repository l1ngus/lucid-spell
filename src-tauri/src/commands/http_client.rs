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
use crate::logic::http_client::build_shared_client;
use crate::state::AppState;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn set_proxy(
    proxy_url: Option<String>,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    // Проверяем текущий прокси под read-lock'ом
    {
        let current = state.proxy_url.read().await;
        if *current == proxy_url {
            return Ok(());
        }
    }

    // Строим НОВЫЙ общий клиент с обновленным прокси
    let new_client = build_shared_client(proxy_url.as_deref())
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    // Обновляем оба поля максимально близко друг к другу
    {
        let mut proxy_guard = state.proxy_url.write().await;
        let mut http_guard = state.http.write().await;
        *proxy_guard = proxy_url;
        *http_guard = new_client;
    }

    Ok(())
}
