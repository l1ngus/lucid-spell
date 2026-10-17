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

//! Service layer of the local OCR feature: status reporting, model
//! installation/removal, initialization and text recognition.

use crate::app_state::{AppState, OcrState};
use crate::dto::{
    OcrDownloadProgress, OcrInstallRequest, OcrLine, OcrModelInfo, OcrPoint, OcrRequest,
    OcrResponse, OcrStatusResponse,
};
use crate::ocr::engine::OcrEngine;
use crate::ocr::error::{ort_err, OcrError};
use crate::ocr::languages::{model_for_language, RecModelSpec, DET_MODEL, REC_MODELS};
use crate::ocr::models::{self, ModelFile};
use base64::Engine as _;
use image::RgbImage;
use ort::environment::Environment;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};

/// Tauri event with [`OcrDownloadProgress`] payload.
const DOWNLOAD_PROGRESS_EVENT: &str = "ocr-download-progress";

/// The directory all OCR models live in.
fn models_dir(state: &OcrState) -> Result<PathBuf, OcrError> {
    state
        .models_dir
        .get()
        .cloned()
        .ok_or(OcrError::ModelsDirMissing)
}

/// Returns the process-wide ONNX Runtime environment, creating it on the first
/// call. ONNX Runtime allows only one environment per process, hence the
/// `OnceLock` in the app state.
fn environment(state: &OcrState) -> Result<Environment, OcrError> {
    if let Some(environment) = state.environment.get() {
        return Ok(environment.clone());
    }

    let environment = ort::init()
        .with_name("lucid-spell-ocr")
        .build()
        .map_err(ort_err)?;
    let _ = state.environment.set(environment);
    Ok(state
        .environment
        .get()
        .expect("the OCR environment was just set")
        .clone())
}

/// Describes which models are downloaded and which sessions are loaded.
fn status(state: &OcrState) -> Result<OcrStatusResponse, OcrError> {
    let models_dir = models_dir(state)?;
    let engine = state
        .engine
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let loaded = engine
        .as_ref()
        .map(OcrEngine::loaded_models)
        .unwrap_or_default();

    let models = REC_MODELS
        .iter()
        .map(|spec| OcrModelInfo {
            id: spec.id.to_string(),
            name: spec.name.to_string(),
            languages: spec.languages.iter().map(|lang| lang.to_string()).collect(),
            installed: models::is_installed(&models::recognition_files(&models_dir, spec)),
            loaded: loaded.iter().any(|id| id == spec.id),
            size_bytes: spec.download_size(),
        })
        .collect();

    Ok(OcrStatusResponse {
        initialized: engine.is_some(),
        models_dir: models_dir.to_string_lossy().into_owned(),
        detection_installed: models::detection_installed(&models_dir),
        detection_loaded: engine.is_some(),
        detection_size_bytes: DET_MODEL.size,
        models,
    })
}

/// Downloads the files of a model, emitting download progress events.
async fn download(
    app: &AppHandle,
    client: &reqwest::Client,
    files: &[ModelFile],
    model_id: &str,
) -> Result<(), OcrError> {
    let mut last_percent = u8::MAX;
    models::download_files(client, files, |file, downloaded| {
        let total = file.size;
        let percent = (downloaded * 100)
            .checked_div(total)
            .map_or(100, |percent| percent.min(100) as u8);
        if percent == last_percent {
            return;
        }
        last_percent = percent;

        let _ = app.emit(
            DOWNLOAD_PROGRESS_EVENT,
            OcrDownloadProgress {
                model: model_id.to_string(),
                file_name: file.file_name(),
                downloaded_bytes: downloaded,
                total_bytes: total,
                progress: if total == 0 {
                    1.0
                } else {
                    (downloaded as f32 / total as f32).min(1.0)
                },
            },
        );
    })
    .await
}

/// Returns the current OCR status: installed models and loaded sessions.
pub async fn get_status(state: &Arc<AppState>) -> Result<OcrStatusResponse, String> {
    status(&state.ocr).map_err(Into::into)
}

/// Downloads the recognition models for the requested languages (including the
/// shared detection model). Nothing is loaded into memory here; call
/// `initialize_ocr` afterwards. Emits `ocr-download-progress` events.
pub async fn install_languages(
    app: &AppHandle,
    request: OcrInstallRequest,
    state: &Arc<AppState>,
) -> Result<OcrStatusResponse, String> {
    let models_dir = models_dir(&state.ocr)?;
    let _guard = state.ocr.install_lock.lock().await;

    models::ensure_directory(&models_dir).await?;
    let client = state.http.client.read().await.clone();

    // Every language needs the shared detection model.
    let detection = models::detection_files(&models_dir);
    download(app, &client, &detection, "detection").await?;

    let mut specs: Vec<&RecModelSpec> = Vec::new();
    for language in &request.languages {
        let spec = model_for_language(language)?;
        if !specs.iter().any(|existing| existing.id == spec.id) {
            specs.push(spec);
        }
    }
    for spec in specs {
        let files = models::recognition_files(&models_dir, spec);
        download(app, &client, &files, spec.id).await?;
    }

    status(&state.ocr).map_err(Into::into)
}

/// Deletes the recognition model of the language (model groups are shared
/// between languages of the same script) and unloads it if it was loaded.
pub async fn uninstall_language(
    language: String,
    state: &Arc<AppState>,
) -> Result<OcrStatusResponse, String> {
    let _guard = state.ocr.install_lock.lock().await;
    let models_dir = models_dir(&state.ocr)?;
    let spec = model_for_language(&language)?;

    let files = models::recognition_files(&models_dir, spec);
    models::remove_files(&files).await?;

    let model_id = spec.id.to_string();
    let state_for_task = Arc::clone(state);
    tauri::async_runtime::spawn_blocking(move || {
        let mut engine = state_for_task
            .ocr
            .engine
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if let Some(engine) = engine.as_mut() {
            engine.unload(&model_id);
        }
    })
    .await
    .map_err(|error| format!("Failed to unload the OCR model: {error}"))?;

    status(&state.ocr).map_err(Into::into)
}

/// Loads the detection model and all installed recognition models into ONNX
/// Runtime sessions. Requires at least the detection model and one recognition
/// model to be installed.
pub async fn initialize(state: &Arc<AppState>) -> Result<OcrStatusResponse, String> {
    let _guard = state.ocr.install_lock.lock().await;
    let models_dir = models_dir(&state.ocr)?;
    if !models::detection_installed(&models_dir) {
        return Err(OcrError::DetectionModelNotInstalled.into());
    }

    let environment = environment(&state.ocr)?;
    let installed = models::installed_recognition_ids(&models_dir);
    let engine = tauri::async_runtime::spawn_blocking(move || {
        OcrEngine::load(&models_dir, &environment, &installed)
    })
    .await
    .map_err(|error| format!("OCR initialization failed: {error}"))?
    .map_err(String::from)?;

    {
        let mut current = state
            .ocr
            .engine
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        *current = Some(engine);
    }

    status(&state.ocr).map_err(Into::into)
}

/// Recognizes all text on the image. The OCR engine has to be initialized and
/// the recognition model for the language installed beforehand.
pub async fn recognize(request: OcrRequest, state: &Arc<AppState>) -> Result<OcrResponse, String> {
    let lang = request.lang.trim().to_lowercase();
    let spec = model_for_language(&lang)?;
    let state_for_task = Arc::clone(state);

    tauri::async_runtime::spawn_blocking(move || -> Result<OcrResponse, OcrError> {
        let image = decode_image(&request)?;
        let (image_width, image_height) = image.dimensions();

        let mut engine = state_for_task
            .ocr
            .engine
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let engine = engine.as_mut().ok_or(OcrError::NotInitialized)?;
        let (text, lines) = engine.recognize(&image, &lang)?;

        Ok(OcrResponse {
            text,
            lines: lines
                .into_iter()
                .map(|line| OcrLine {
                    text: line.text,
                    confidence: line.confidence,
                    bbox: line
                        .bbox
                        .iter()
                        .map(|point| OcrPoint {
                            x: point.x,
                            y: point.y,
                        })
                        .collect(),
                })
                .collect(),
            image_width,
            image_height,
            lang,
            model: spec.id.to_string(),
        })
    })
    .await
    .map_err(|error| format!("OCR request failed: {error}"))?
    .map_err(String::from)
}

/// Reads the image from either an absolute path or base64 data.
fn decode_image(request: &OcrRequest) -> Result<RgbImage, OcrError> {
    let bytes = match (&request.image_base64, &request.image_path) {
        (Some(_), Some(_)) => {
            return Err(OcrError::InvalidInput(
                "Provide either `imageBase64` or `imagePath`, not both".to_string(),
            ));
        }
        (None, None) => {
            return Err(OcrError::InvalidInput(
                "Either `imageBase64` or `imagePath` must be provided".to_string(),
            ));
        }
        (Some(encoded), None) => decode_base64_image(encoded)?,
        (None, Some(path)) => std::fs::read(path).map_err(|error| {
            OcrError::InvalidInput(format!("Failed to read the image file '{path}': {error}"))
        })?,
    };

    let image =
        image::load_from_memory(&bytes).map_err(|error| OcrError::Image(error.to_string()))?;
    Ok(image.to_rgb8())
}

/// Decodes base64 image data. Both plain base64 and `data:` URLs are accepted,
/// with the standard or the URL-safe alphabet and with or without padding.
fn decode_base64_image(encoded: &str) -> Result<Vec<u8>, OcrError> {
    use base64::engine::general_purpose::{STANDARD, STANDARD_NO_PAD, URL_SAFE, URL_SAFE_NO_PAD};

    let data = encoded.trim();
    let data = match data.split_once(";base64,") {
        Some((header, rest)) if header.starts_with("data:") => rest,
        _ => data,
    };
    let compact: String = data.chars().filter(|char| !char.is_whitespace()).collect();

    STANDARD
        .decode(&compact)
        .or_else(|_| STANDARD_NO_PAD.decode(&compact))
        .or_else(|_| URL_SAFE.decode(&compact))
        .or_else(|_| URL_SAFE_NO_PAD.decode(&compact))
        .map_err(|error| {
            OcrError::InvalidInput(format!("Failed to decode the base64 image: {error}"))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(base64: Option<&str>, path: Option<&str>) -> OcrRequest {
        OcrRequest {
            image_base64: base64.map(str::to_string),
            image_path: path.map(str::to_string),
            lang: "eng".to_string(),
        }
    }

    #[test]
    fn base64_is_decoded_with_data_urls_and_url_safe_alphabet() {
        // "lucid" in the three supported encodings.
        assert_eq!(decode_base64_image("bHVjaWQ=").unwrap(), b"lucid".to_vec());
        assert_eq!(
            decode_base64_image("data:image/png;base64,bHVjaWQ=").unwrap(),
            b"lucid".to_vec()
        );
        assert_eq!(
            decode_base64_image("data:image/png;base64,bHVjaWQ").unwrap(),
            b"lucid".to_vec()
        );
        // 0xff 0xef is `/+8=` with the standard alphabet and `_-8=` with the
        // URL-safe one.
        assert_eq!(decode_base64_image("_-8=").unwrap(), vec![0xff, 0xef]);
        assert!(decode_base64_image("not base64!").is_err());
    }

    #[test]
    fn image_requests_validate_their_source() {
        assert!(decode_image(&request(None, None)).is_err());
        assert!(decode_image(&request(Some("bHVjaWQ="), Some("/tmp/x.png"))).is_err());
    }
}
