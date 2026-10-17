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

use crate::infra::http_client::build_shared_client;
use crate::ocr::engine::OcrEngine;
use msedge_tts::voice::Voice;
use ort::environment::Environment;
use reqwest::Client as HttpClient;
use rodio::mixer::Mixer;
use rodio::OutputStream;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// rodio::OutputStream is !Send on macOS (cpal CoreAudio listener callback).
/// Safe because the stream is created once in `setup()` and never moved; only
/// the mixer (which is truly Send) is accessed from commands.
pub(crate) struct OutputStreamHandle(pub(crate) Option<OutputStream>);

unsafe impl Send for OutputStreamHandle {}

#[derive(Default, Clone)]
pub struct LlmSettings {
    pub profile_id: String,
    pub api_url: String,
    pub model: String,
    pub temperature: f32,
}

/// Shared HTTP client plus the proxy URL it was built with.
/// Both fields are always updated together via [`HttpState::set_proxy`],
/// so the client never disagrees with the configured proxy.
pub struct HttpState {
    pub client: RwLock<HttpClient>, //both for async-openai and reqwest requests
    pub proxy_url: RwLock<Option<String>>,
}

impl Default for HttpState {
    fn default() -> Self {
        Self {
            client: RwLock::new(HttpClient::new()),
            proxy_url: RwLock::new(None),
        }
    }
}

impl HttpState {
    pub async fn set_proxy(&self, proxy_url: Option<String>) -> Result<(), String> {
        // Проверяем текущий прокси под read-lock'ом
        {
            let current = self.proxy_url.read().await;
            if *current == proxy_url {
                return Ok(());
            }
        }

        // Строим НОВЫЙ общий клиент с обновленным прокси
        let new_client = build_shared_client(proxy_url.as_deref())
            .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

        // Обновляем оба поля максимально близко друг к другу
        {
            let mut proxy_guard = self.proxy_url.write().await;
            let mut client_guard = self.client.write().await;
            *proxy_guard = proxy_url;
            *client_guard = new_client;
        }

        Ok(())
    }
}

/// TTS audio output and the cached Edge voices list.
#[derive(Default)]
pub struct SpeechState {
    pub audio_mixer: Mutex<Option<Arc<Mixer>>>,
    pub _audio_stream: Mutex<Option<OutputStreamHandle>>,
    pub voices: RwLock<Vec<Voice>>,
}

impl SpeechState {
    /// Creates the default audio output stream.
    /// Must be called once at startup from a synchronous context (Tauri `setup`).
    pub fn init_audio(&self) {
        if let Ok(stream) = rodio::OutputStreamBuilder::open_default_stream() {
            let mixer = stream.mixer().clone();

            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                *self._audio_stream.lock().await = Some(OutputStreamHandle(Some(stream)));
                *self.audio_mixer.lock().await = Some(Arc::new(mixer));
            });
        }
    }

    /// Loads the Edge TTS voices list. Meant to be spawned in the background —
    /// never blocks startup.
    pub async fn load_voices(&self) {
        match msedge_tts::voice::tokio_runtime::get_voices_list_async().await {
            Ok(voices) => {
                *self.voices.write().await = voices;
            }
            Err(e) => {
                eprintln!("Failed to load voices: {e}");
            }
        }
    }
}

/// Local OCR: where the models live, the ONNX Runtime environment and the
/// loaded sessions.
///
/// Models are downloaded on demand. Once the detector and a recognition model
/// are installed, the frontend initializes them on startup and after install.
/// `ocr_image` fails with a clear error until the engine is initialized.
pub struct OcrState {
    /// Directory with the downloaded models (`<app data dir>/ocr-models`).
    /// Set once in the Tauri `setup` hook.
    pub models_dir: std::sync::OnceLock<PathBuf>,
    /// ONNX Runtime allows a single environment per process; it is created on
    /// the first `initialize_ocr` call.
    pub environment: std::sync::OnceLock<Environment>,
    /// Loaded ONNX sessions, `None` until OCR is initialized.
    pub engine: std::sync::Mutex<Option<OcrEngine>>,
    /// Serializes model installation, removal and initialization.
    pub install_lock: Mutex<()>,
}

impl Default for OcrState {
    fn default() -> Self {
        Self {
            models_dir: std::sync::OnceLock::new(),
            environment: std::sync::OnceLock::new(),
            engine: std::sync::Mutex::new(None),
            install_lock: Mutex::new(()),
        }
    }
}

#[derive(Default)]
pub struct AppState {
    pub http: HttpState,
    pub llm: RwLock<LlmSettings>,
    pub ocr: OcrState,
    pub speech: SpeechState,
}
