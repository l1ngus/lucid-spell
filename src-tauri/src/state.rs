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

use msedge_tts::voice::Voice;
use reqwest::Client as HttpClient;
use rodio::mixer::Mixer;
use rodio::OutputStream;
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

pub struct AppState {
    pub http: RwLock<HttpClient>, //both for async-openai and reqwest requests
    pub proxy_url: RwLock<Option<String>>,
    pub llm: RwLock<LlmSettings>,

    pub audio_mixer: Mutex<Option<Arc<Mixer>>>,
    pub _audio_stream: Mutex<Option<OutputStreamHandle>>,
    pub voices: RwLock<Vec<Voice>>,
}
