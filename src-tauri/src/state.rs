use async_openai::{config::OpenAIConfig, Client};
use msedge_tts::voice::Voice;
use rodio::mixer::Mixer;
use rodio::OutputStream;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// rodio::OutputStream is !Send on macOS (cpal CoreAudio listener callback).
/// Safe because the stream is created once in `setup()` and never moved; only
/// the mixer (which is truly Send) is accessed from commands.
pub(crate) struct OutputStreamHandle(pub(crate) Option<OutputStream>);

unsafe impl Send for OutputStreamHandle {}

pub struct AppState {
    pub openai_client: Mutex<Option<Client<OpenAIConfig>>>,
    pub audio_mixer: Mutex<Option<Arc<Mixer>>>,
    pub _audio_stream: Mutex<Option<OutputStreamHandle>>,
    pub voices: RwLock<Vec<Voice>>,
}
