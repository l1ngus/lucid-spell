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

use serde::{Deserialize, Serialize};
use specta::Type;

/// Request to recognize text on an image.
/// Exactly one of `image_base64` / `image_path` must be provided.
#[derive(Debug, Clone, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OcrRequest {
    /// Base64-encoded image, either raw base64 or a `data:image/...;base64,...` URL.
    pub image_base64: Option<String>,
    /// Absolute path to an image file on disk.
    pub image_path: Option<String>,
    /// Language of the text on the image (ISO 639-3, e.g. `eng`).
    pub lang: String,
}

#[derive(Debug, Clone, Serialize, Type)]
pub struct OcrPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OcrLine {
    pub text: String,
    pub confidence: f32,
    /// Detected text quadrilateral in original image pixels (clockwise).
    pub bbox: Vec<OcrPoint>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OcrResponse {
    /// All recognized lines joined with `\n`.
    pub text: String,
    pub lines: Vec<OcrLine>,
    pub image_width: u32,
    pub image_height: u32,
    /// Resolved language (ISO 639-3).
    pub lang: String,
    /// Recognition model that was used (e.g. `english`).
    pub model: String,
}

/// State of a single recognition model (language group).
#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OcrModelInfo {
    /// Model group id (e.g. `english`, `eslav`).
    pub id: String,
    /// Human readable name.
    pub name: String,
    /// ISO 639-3 codes covered by this model.
    pub languages: Vec<String>,
    /// Recognition model and dictionary are downloaded.
    pub installed: bool,
    /// Model is loaded into memory (after `initialize_ocr`).
    pub loaded: bool,
    /// Download size of the recognition model + dictionary.
    // u64 is a forbidden BigInt in TS; file sizes fit f64 exactly
    #[specta(type = f64)]
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OcrStatusResponse {
    /// ONNX sessions are loaded: `ocr_image` can be called.
    pub initialized: bool,
    pub models_dir: String,
    pub detection_installed: bool,
    pub detection_loaded: bool,
    // u64 is a forbidden BigInt in TS; file sizes fit f64 exactly
    #[specta(type = f64)]
    pub detection_size_bytes: u64,
    pub models: Vec<OcrModelInfo>,
}

#[derive(Debug, Clone, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OcrInstallRequest {
    /// ISO 639-3 language codes to install (e.g. `["eng", "rus"]`).
    pub languages: Vec<String>,
}

/// Payload of the `ocr-download-progress` event.
#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OcrDownloadProgress {
    /// Model group id (`english`, ...) or `detection`.
    pub model: String,
    pub file_name: String,
    // u64 is a forbidden BigInt in TS; byte counts fit f64 exactly
    #[specta(type = f64)]
    pub downloaded_bytes: u64,
    #[specta(type = f64)]
    pub total_bytes: u64,
    /// 0.0..=1.0
    pub progress: f32,
}
