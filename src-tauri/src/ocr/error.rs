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

use thiserror::Error;

/// Errors of the local OCR service. Command handlers convert them into
/// user-facing strings via [`OcrError::to_string`].
#[derive(Debug, Error)]
pub enum OcrError {
    #[error("File system error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Network error: {0}")]
    Http(String),

    #[error("Failed to decode image: {0}")]
    Image(String),

    #[error("ONNX Runtime error: {0}")]
    Ort(String),

    #[error("{0}")]
    InvalidInput(String),

    #[error("OCR models directory is not available")]
    ModelsDirMissing,

    #[error("OCR is not initialized. Install models and call `initialize_ocr` first")]
    NotInitialized,

    #[error("The text detection model is not installed")]
    DetectionModelNotInstalled,

    #[error("No recognition models installed. Install at least one language first")]
    NoRecognitionModels,

    #[error("Language '{lang}' is not supported by the local OCR models")]
    UnsupportedLanguage { lang: String },

    #[error("Recognition model '{model}' (language '{lang}') is not installed")]
    ModelNotInstalled { lang: String, model: String },

    #[error(
        "Downloaded file '{file}' has unexpected size: expected {expected} bytes, got {actual}"
    )]
    DownloadSizeMismatch {
        file: String,
        expected: u64,
        actual: u64,
    },

    #[error("Failed to download '{url}': {reason}")]
    Download { url: String, reason: String },
}

impl From<OcrError> for String {
    fn from(value: OcrError) -> Self {
        value.to_string()
    }
}

/// Helper for mapping `ort` errors, which are not `Send`-friendly to store directly.
pub(crate) fn ort_err(error: impl std::fmt::Display) -> OcrError {
    OcrError::Ort(error.to_string())
}
