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

mod commands;
mod llm;
mod logic;
mod models;
mod state;
mod translation;

use specta_typescript::Typescript;
use state::AppState;
use std::sync::Arc;
use tauri::Manager;
use tauri_specta::{collect_commands, Builder};
use tokio::sync::{Mutex, RwLock};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Both `ring` (reqwest 0.12) and `aws-lc-rs` (msedge-tts -> reqwest 0.13)
    // are compiled into rustls, so pick a provider explicitly.
    let _ = rustls::crypto::ring::default_provider().install_default();
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    let builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            commands::llm::set_llm_config,
            // commands::llm::ask_llm,
            commands::tts::speak,
            commands::tts::get_voices,
            commands::keys::save_profile_api_key,
            commands::keys::check_profile_api_key,
            commands::keys::remove_profile_api_key,
            commands::http_client::set_proxy,
            commands::translation::translate
        ])
        .typ::<models::ChatMessage>();
    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(Arc::new(AppState {
            http: RwLock::new(reqwest::Client::new()),
            proxy_url: RwLock::new(None),
            llm: RwLock::new(state::LlmSettings::default()),
            audio_mixer: Mutex::new(None),
            _audio_stream: Mutex::new(None),
            voices: tokio::sync::RwLock::new(Vec::new()),
        }))
        .setup(|app| {
            let state = app.try_state::<Arc<AppState>>().unwrap();

            // Load voices in background — не блокируем запуск
            let state_arc = Arc::clone(&state);
            tauri::async_runtime::spawn(async move {
                match msedge_tts::voice::tokio_runtime::get_voices_list_async().await {
                    Ok(voices) => {
                        *state_arc.voices.write().await = voices;
                    }
                    Err(e) => {
                        eprintln!("Failed to load voices: {e}");
                    }
                }
            });

            if let Ok(stream) = rodio::OutputStreamBuilder::open_default_stream() {
                let mixer = stream.mixer().clone();

                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let mut audio_stream = state._audio_stream.lock().await;
                    let mut audio_mixer = state.audio_mixer.lock().await;

                    *audio_stream = Some(state::OutputStreamHandle(Some(stream)));
                    *audio_mixer = Some(Arc::new(mixer));
                });
            }

            Ok(())
        })
        .invoke_handler(builder.invoke_handler())
        .invoke_handler(tauri::generate_handler![
            commands::lang::detect_language,
            commands::llm::set_llm_config,
            // commands::llm::ask_llm,
            commands::tts::speak,
            commands::tts::get_voices,
            commands::keys::save_profile_api_key,
            commands::keys::check_profile_api_key,
            commands::keys::remove_profile_api_key,
            commands::http_client::set_proxy,
            commands::translation::translate
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
