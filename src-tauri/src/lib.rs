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

mod app_state;
mod commands;
mod dto;
mod infra;
mod remote_llm;
mod translation;

use app_state::AppState;
use specta_typescript::Typescript;
use std::sync::Arc;
use tauri::Manager;
use tauri_specta::{collect_commands, Builder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("failed to install aws-lc-rs provider");
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    let builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            commands::llm::set_llm_config,
            commands::tts::speak,
            commands::tts::get_voices,
            commands::keys::save_profile_api_key,
            commands::keys::check_profile_api_key,
            commands::keys::remove_profile_api_key,
            commands::http_client::set_proxy,
            commands::translation::translate,
            commands::translation::get_other_translations
        ])
        .typ::<dto::ChatMessage>();
    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(Arc::new(AppState::default()))
        .setup(|app| {
            let state = app.try_state::<Arc<AppState>>().unwrap();

            // Load voices in background — не блокируем запуск
            let state_arc = Arc::clone(&state);
            tauri::async_runtime::spawn(async move {
                state_arc.speech.load_voices().await;
            });

            state.speech.init_audio();

            Ok(())
        })
        .invoke_handler(builder.invoke_handler())
        .invoke_handler(tauri::generate_handler![
            commands::lang::detect_language,
            commands::llm::set_llm_config,
            commands::tts::speak,
            commands::tts::get_voices,
            commands::keys::save_profile_api_key,
            commands::keys::check_profile_api_key,
            commands::keys::remove_profile_api_key,
            commands::http_client::set_proxy,
            commands::translation::translate,
            commands::translation::get_other_translations
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
