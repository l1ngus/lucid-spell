mod commands;
mod logic;
mod models;
mod state;

use specta_typescript::Typescript;
use state::AppState;
use std::sync::Arc;
use tauri::Manager;
use tauri_specta::{collect_commands, Builder};
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    let builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            commands::llm::set_llm_config,
            commands::llm::ask_llm,
            commands::tts::speak,
            commands::tts::get_voices,
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
            openai_client: Mutex::new(None),
            audio_mixer: Mutex::new(None),
            _audio_stream: Mutex::new(None),
            voices: tokio::sync::RwLock::new(Vec::new()),
        }))
        .setup(|app| {
            let state = app.try_state::<Arc<AppState>>().unwrap();

            // Load voices in background — не блокируем запуск
            let state_arc = Arc::clone(&state);
            tauri::async_runtime::spawn(async move {
                match msedge_tts::voice::get_voices_list_async().await {
                    Ok(voices) => {
                        *state_arc.voices.write().await = voices;
                    }
                    Err(e) => {
                        eprintln!("Failed to load voices: {e}");
                    }
                }
            });

            // Init audio in background — open_default_stream can hang on VMs without audio
            let state_arc = Arc::clone(&state);
            tauri::async_runtime::spawn(async move {
                match tokio::task::spawn_blocking(|| {
                    rodio::OutputStreamBuilder::open_default_stream()
                })
                .await
                {
                    Ok(Ok(stream)) => {
                        let mixer = stream.mixer().clone();
                        let mut audio_stream = state_arc._audio_stream.lock().await;
                        let mut audio_mixer = state_arc.audio_mixer.lock().await;
                        *audio_stream = Some(state::OutputStreamHandle(Some(stream)));
                        *audio_mixer = Some(Arc::new(mixer));
                    }
                    Ok(Err(e)) => eprintln!("Audio init failed: {e}"),
                    Err(e) => eprintln!("Audio init task cancelled: {e}"),
                }
            });

            Ok(())
        })
        .invoke_handler(builder.invoke_handler())
        .invoke_handler(tauri::generate_handler![
            commands::lang::detect_language,
            commands::llm::set_llm_config,
            commands::llm::ask_llm,
            commands::tts::speak,
            commands::tts::get_voices,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
