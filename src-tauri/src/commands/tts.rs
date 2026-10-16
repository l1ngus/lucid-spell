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

use crate::app_state::AppState;
use crate::infra::play_audio;
use msedge_tts::tts::client::tokio_runtime::connect_async;
use msedge_tts::tts::SpeechConfig;
use specta;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn get_voices(state: State<'_, Arc<AppState>>) -> Result<Vec<String>, String> {
    let voices = state.speech.voices.read().await;
    Ok(voices.iter().filter_map(|v| v.short_name.clone()).collect())
}

#[tauri::command]
#[specta::specta]
pub async fn speak(
    text: String,
    voice: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let voices = state.speech.voices.read().await;
    let voice_info = voices
        .iter()
        .find(|v| v.short_name.as_ref() == Some(&voice))
        .ok_or_else(|| format!("Voice '{}' not found", voice))?;

    let config = SpeechConfig::from(voice_info);
    let mut tts = connect_async()
        .await
        .map_err(|e| format!("Failed to connect to TTS: {}", e))?;
    let audio = tts
        .synthesize(&text, &config)
        .await
        .map_err(|e| format!("Failed to synthesize speech: {}", e))?;

    drop(voices);

    // Получаем микшер из AppState
    let audio_mixer = state.speech.audio_mixer.lock().await;
    let mixer = audio_mixer.as_ref().ok_or("Audio not initialized")?.clone();
    drop(audio_mixer);

    play_audio(audio.audio_bytes, mixer)?;

    Ok(())
}
