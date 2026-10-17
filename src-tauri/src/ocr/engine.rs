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

//! OCR engine: the detection session plus one recognition session per
//! installed model group. Runs the full detection + recognition pipeline.

use crate::ocr::det::DetModel;
use crate::ocr::error::OcrError;
use crate::ocr::geometry::{crop_quad, Point};
use crate::ocr::languages::{find_model, model_for_language, DET_MODEL};
use crate::ocr::rec::RecModel;
use image::RgbImage;
use ort::environment::Environment;
use std::collections::HashMap;
use std::path::Path;

/// One recognized line of text.
pub struct RecognizedLine {
    pub text: String,
    pub confidence: f32,
    pub bbox: [Point; 4],
}

/// The loaded ONNX sessions. Created by `initialize_ocr` and kept in the app
/// state until the user uninstalls a model or restarts the app.
pub struct OcrEngine {
    detection: DetModel,
    recognition: HashMap<String, RecModel>,
}

impl OcrEngine {
    /// Loads the detection model and every recognition model that was both
    /// requested and found on disk.
    pub fn load(
        models_dir: &Path,
        environment: &Environment,
        installed_models: &[String],
    ) -> Result<Self, OcrError> {
        let detection_path = models_dir.join(DET_MODEL.file);
        if !detection_path.is_file() {
            return Err(OcrError::DetectionModelNotInstalled);
        }
        let detection = DetModel::load(&detection_path, environment)?;

        let mut recognition = HashMap::new();
        for id in installed_models {
            let spec = find_model(id).ok_or_else(|| {
                OcrError::InvalidInput(format!("Unknown recognition model '{id}'"))
            })?;
            let model_path = models_dir.join(spec.rec_file);
            let dictionary_path = models_dir.join(spec.dict_file);
            if !model_path.is_file() || !dictionary_path.is_file() {
                continue;
            }
            recognition.insert(
                spec.id.to_string(),
                RecModel::load(spec.id, &model_path, &dictionary_path, environment)?,
            );
        }
        if recognition.is_empty() {
            return Err(OcrError::NoRecognitionModels);
        }

        Ok(Self {
            detection,
            recognition,
        })
    }

    /// Ids of the recognition models that are currently loaded.
    pub fn loaded_models(&self) -> Vec<String> {
        let mut models: Vec<String> = self.recognition.keys().cloned().collect();
        models.sort();
        models
    }

    /// Drops a recognition model from memory (used when it is uninstalled).
    pub fn unload(&mut self, model_id: &str) {
        self.recognition.remove(model_id);
    }

    /// Detects and recognizes all text on the image for the given language.
    pub fn recognize(
        &mut self,
        image: &RgbImage,
        lang: &str,
    ) -> Result<(String, Vec<RecognizedLine>), OcrError> {
        let spec = model_for_language(lang)?;
        if !self.recognition.contains_key(spec.id) {
            return Err(OcrError::ModelNotInstalled {
                lang: lang.to_string(),
                model: spec.id.to_string(),
            });
        }

        let boxes = self.detection.detect(image)?;
        let recognition = self
            .recognition
            .get_mut(spec.id)
            .expect("recognition model presence was checked above");

        let mut lines = Vec::with_capacity(boxes.len());
        for bbox in boxes {
            let Some((crop, _)) = crop_quad(image, bbox) else {
                continue;
            };
            let (text, confidence) = recognition.recognize(&crop)?;
            if text.trim().is_empty() {
                continue;
            }
            lines.push(RecognizedLine {
                text,
                confidence,
                bbox,
            });
        }

        let text = lines
            .iter()
            .map(|line| line.text.as_str())
            .collect::<Vec<&str>>()
            .join("\n");
        Ok((text, lines))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn e2e_environment() -> &'static Environment {
        static ENVIRONMENT: OnceLock<Environment> = OnceLock::new();
        ENVIRONMENT.get_or_init(|| {
            ort::init()
                .with_name("lucid-spell-ocr-e2e")
                .build()
                .expect("failed to create the ONNX Runtime environment")
        })
    }

    /// End-to-end pipeline run against real PaddleOCR models. Skipped by
    /// default; point it at a directory containing `det_ch_PP-OCRv4.onnx`,
    /// `rec_english.onnx` and `dict_english.txt`, plus an image with legible
    /// English text:
    ///
    /// ```sh
    /// OCR_E2E_MODELS=/path/to/models OCR_E2E_IMAGE=/path/to/image.png \
    ///     cargo test ocr::engine -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore = "requires downloaded models, see OCR_E2E_MODELS / OCR_E2E_IMAGE"]
    fn recognizes_real_image() {
        let Some(models_dir) = std::env::var_os("OCR_E2E_MODELS") else {
            return;
        };
        let Some(image_path) = std::env::var_os("OCR_E2E_IMAGE") else {
            return;
        };

        let models_dir = Path::new(&models_dir);
        let mut engine =
            OcrEngine::load(models_dir, e2e_environment(), &["english".to_string()]).unwrap();
        let image = image::open(&image_path)
            .expect("failed to open the test image")
            .to_rgb8();

        let (text, lines) = engine.recognize(&image, "eng").unwrap();
        for line in &lines {
            println!(
                "{:.3}  {}  {:?}",
                line.confidence,
                line.text,
                line.bbox.map(|point| [point.x, point.y])
            );
        }
        println!("---\n{text}");

        let lowered = text.to_lowercase();
        assert!(
            lowered.contains("hello world"),
            "expected 'Hello world!' in:\n{text}"
        );
        assert!(
            lowered.contains("quick brown fox"),
            "expected the pangram in:\n{text}"
        );
        assert!(lowered.contains("2026"), "expected '2026' in:\n{text}");
    }
}
