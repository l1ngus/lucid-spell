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

//! Local OCR: PaddleOCR text detection + recognition models running on ONNX
//! Runtime through `ort`.
//!
//! The pipeline is PP-OCRv4 detection (`det`) followed by a script-specific
//! recognition model (`rec`, picked by the requested language). Models are
//! downloaded on demand from Hugging Face into the app data directory and are
//! only loaded into memory when the user explicitly initializes OCR.

pub mod db_postprocess;
pub mod det;
pub mod engine;
pub mod error;
pub mod geometry;
pub mod languages;
pub mod models;
pub mod preprocess;
pub mod rec;
pub mod service;

pub use error::OcrError;

use ort::environment::Environment;
use ort::session::builder::GraphOptimizationLevel;
use ort::session::Session;
use std::path::Path;

/// Creates an ONNX Runtime session for a model file with graph optimizations
/// enabled. Used for both detection and recognition models.
pub(crate) fn create_session(path: &Path, environment: &Environment) -> Result<Session, OcrError> {
    Session::builder(environment)
        .map_err(error::ort_err)?
        .with_optimization_level(GraphOptimizationLevel::Level3)
        .map_err(error::ort_err)?
        .commit_from_file(path)
        .map_err(error::ort_err)
}
