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
use crate::dto::{OcrInstallRequest, OcrRequest, OcrResponse, OcrStatusResponse};
use std::sync::Arc;
use tauri::{AppHandle, State};

/// Returns which OCR models are downloaded and which ONNX sessions are loaded.
#[tauri::command]
#[specta::specta]
pub async fn get_ocr_status(state: State<'_, Arc<AppState>>) -> Result<OcrStatusResponse, String> {
    crate::ocr::service::get_status(state.inner()).await
}

/// Downloads recognition models for the given languages (ISO 639-3) together
/// with the shared text detection model. Emits `ocr-download-progress` events
/// while downloading; nothing is loaded into memory until `initialize_ocr`.
#[tauri::command]
#[specta::specta]
pub async fn install_ocr_languages(
    app: AppHandle,
    request: OcrInstallRequest,
    state: State<'_, Arc<AppState>>,
) -> Result<OcrStatusResponse, String> {
    crate::ocr::service::install_languages(&app, request, state.inner()).await
}

/// Deletes the recognition model of the given language (ISO 639-3). Model
/// groups are shared between languages of the same script, so other languages
/// may become unavailable as well.
#[tauri::command]
#[specta::specta]
pub async fn uninstall_ocr_language(
    language: String,
    state: State<'_, Arc<AppState>>,
) -> Result<OcrStatusResponse, String> {
    crate::ocr::service::uninstall_language(language, state.inner()).await
}

/// Deletes the shared OCR text detection model and unloads all OCR sessions.
#[tauri::command]
#[specta::specta]
pub async fn uninstall_ocr_detection_model(
    state: State<'_, Arc<AppState>>,
) -> Result<OcrStatusResponse, String> {
    crate::ocr::service::uninstall_detection_model(state.inner()).await
}

/// Loads all installed models into ONNX Runtime sessions. This is the first
/// call that allocates memory for the models; `ocr_image` fails before it.
#[tauri::command]
#[specta::specta]
pub async fn initialize_ocr(state: State<'_, Arc<AppState>>) -> Result<OcrStatusResponse, String> {
    crate::ocr::service::initialize(state.inner()).await
}

/// Recognizes all text on an image (base64 or a file path) and returns the
/// recognized lines with their confidence and coordinates.
#[tauri::command]
#[specta::specta]
pub async fn ocr_image(
    request: OcrRequest,
    state: State<'_, Arc<AppState>>,
) -> Result<OcrResponse, String> {
    crate::ocr::service::recognize(request, state.inner()).await
}
