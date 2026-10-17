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

//! Text recognition model (PaddleOCR CRNN + CTC) on ONNX Runtime.

use crate::ocr::create_session;
use crate::ocr::error::{ort_err, OcrError};
use crate::ocr::preprocess::prepare_rec_input;
use image::RgbImage;
use ort::environment::Environment;
use ort::session::Session;
use ort::value::{Tensor, ValueType};
use std::path::Path;

/// Loaded recognition session together with the model's character set.
pub struct RecModel {
    session: Session,
    /// The model group id (e.g. `english`), used in error messages.
    id: String,
    /// CTC vocabulary: `characters[0]` is the blank symbol, the rest map
    /// classes to text.
    characters: Vec<String>,
}

impl RecModel {
    /// Loads the recognition model and reads its dictionary. The number of
    /// model classes is checked against the dictionary so that a mismatching
    /// pair fails right away.
    pub fn load(
        id: &str,
        model_path: &Path,
        dict_path: &Path,
        environment: &Environment,
    ) -> Result<Self, OcrError> {
        let session = create_session(model_path, environment)?;
        let classes = model_classes(&session)?;
        let dictionary = read_dictionary(dict_path)?;
        let characters = build_vocabulary(&dictionary, classes)?;

        Ok(Self {
            session,
            id: id.to_string(),
            characters,
        })
    }

    /// Recognizes one text line crop. Returns the decoded text and its average
    /// character confidence.
    pub fn recognize(&mut self, image: &RgbImage) -> Result<(String, f32), OcrError> {
        let input = prepare_rec_input(image, crate::ocr::languages::REC_INPUT_HEIGHT);
        let (input_height, input_width) = (input.height, input.width);
        let tensor = Tensor::from_array(([1usize, 3, input_height, input_width], input.tensor))
            .map_err(ort_err)?;

        let outputs = self.session.run(ort::inputs![tensor]).map_err(ort_err)?;
        let (shape, probabilities) = outputs[0].try_extract_tensor::<f32>().map_err(ort_err)?;
        if shape.len() != 3 {
            return Err(OcrError::Ort(format!(
                "Unexpected recognition model output shape: {shape:?}"
            )));
        }

        let time_steps = shape[1] as usize;
        let classes = shape[2] as usize;
        if classes != self.characters.len() {
            return Err(OcrError::Ort(format!(
                "Recognition model '{}' returned {classes} classes, but its dictionary produces {}",
                self.id,
                self.characters.len()
            )));
        }
        if probabilities.len() < time_steps * classes {
            return Err(OcrError::Ort(
                "Recognition model output is smaller than its shape suggests".to_string(),
            ));
        }

        Ok(ctc_greedy_decode(
            probabilities,
            time_steps,
            classes,
            &self.characters,
        ))
    }
}

/// Number of classes of a recognition model, taken from its output shape.
fn model_classes(session: &Session) -> Result<usize, OcrError> {
    let output = session
        .outputs()
        .first()
        .ok_or_else(|| OcrError::Ort("Recognition model has no outputs".to_string()))?;
    match output.dtype() {
        ValueType::Tensor { shape, .. } => shape
            .get(2)
            .copied()
            .filter(|classes| *classes > 0)
            .map(|classes| classes as usize)
            .ok_or_else(|| {
                OcrError::Ort(
                    "Cannot determine the class count of the recognition model".to_string(),
                )
            }),
        _ => Err(OcrError::Ort(format!(
            "Unexpected recognition model output type: {:?}",
            output.dtype()
        ))),
    }
}

/// Reads a PaddleOCR dictionary file; one character per line.
fn read_dictionary(path: &Path) -> Result<Vec<String>, OcrError> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents.lines().map(str::to_string).collect())
}

/// Builds the CTC vocabulary from a dictionary. PaddleOCR dictionaries do not
/// contain the CTC blank or the space character: models with `d + 2` classes
/// use blank + dictionary + space (the common case, `use_space_char`), models
/// with `d + 1` classes use blank + dictionary.
fn build_vocabulary(dictionary: &[String], classes: usize) -> Result<Vec<String>, OcrError> {
    let extra = classes.checked_sub(dictionary.len());
    match extra {
        Some(2) => {
            let mut characters = Vec::with_capacity(classes);
            characters.push(String::new()); // CTC blank
            characters.extend(dictionary.iter().cloned());
            characters.push(" ".to_string());
            Ok(characters)
        }
        Some(1) => {
            let mut characters = Vec::with_capacity(classes);
            characters.push(String::new()); // CTC blank
            characters.extend(dictionary.iter().cloned());
            Ok(characters)
        }
        _ => Err(OcrError::Ort(format!(
            "Recognition model has {classes} classes, but its dictionary has {} entries",
            dictionary.len()
        ))),
    }
}

/// Greedy CTC decoding: take the most likely class per time step, drop blanks
/// and immediate repetitions. The confidence is the mean probability of the
/// characters that were kept.
fn ctc_greedy_decode(
    probabilities: &[f32],
    time_steps: usize,
    classes: usize,
    characters: &[String],
) -> (String, f32) {
    let mut text = String::new();
    let mut confidence_sum = 0.0f32;
    let mut confidence_count = 0usize;
    let mut previous = usize::MAX;

    for time_step in 0..time_steps {
        let row = &probabilities[time_step * classes..(time_step + 1) * classes];
        let (mut best_index, mut best_probability) = (0usize, f32::MIN);
        for (index, probability) in row.iter().enumerate() {
            if *probability > best_probability {
                best_index = index;
                best_probability = *probability;
            }
        }

        if best_index != 0 && best_index != previous {
            text.push_str(&characters[best_index]);
            confidence_sum += best_probability;
            confidence_count += 1;
        }
        previous = best_index;
    }

    let confidence = if confidence_count == 0 {
        0.0
    } else {
        confidence_sum / confidence_count as f32
    };
    (text, confidence)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vocabulary(count: usize) -> Vec<String> {
        (0..count).map(|index| format!("c{index}")).collect()
    }

    fn one_hot_row(classes: usize, index: usize) -> Vec<f32> {
        let mut row = vec![0.0f32; classes];
        row[index] = 1.0;
        row
    }

    #[test]
    fn vocabulary_with_space_and_without_it() {
        let dictionary: Vec<String> = ["a", "b", "c"].iter().map(|s| s.to_string()).collect();

        let with_space = build_vocabulary(&dictionary, 5).unwrap();
        assert_eq!(with_space, vec!["", "a", "b", "c", " "]);

        let without_space = build_vocabulary(&dictionary, 4).unwrap();
        assert_eq!(without_space, vec!["", "a", "b", "c"]);

        assert!(build_vocabulary(&dictionary, 6).is_err());
        assert!(build_vocabulary(&dictionary, 2).is_err());
    }

    #[test]
    fn decode_drops_blanks_and_repetitions() {
        let characters = vocabulary(4);
        // a a blank a b b b
        let sequence = [1usize, 1, 0, 1, 2, 2, 2];
        let classes = 4;
        let mut probabilities = Vec::new();
        for index in sequence {
            probabilities.extend(one_hot_row(classes, index));
        }
        let (text, confidence) =
            ctc_greedy_decode(&probabilities, sequence.len(), classes, &characters);
        assert_eq!(text, "c1c1c2");
        assert!((confidence - 1.0).abs() < 1e-6);
    }

    #[test]
    fn decode_without_characters_has_zero_confidence() {
        let characters = vocabulary(3);
        let probabilities = one_hot_row(3, 0)
            .into_iter()
            .chain(one_hot_row(3, 0))
            .collect::<Vec<f32>>();
        let (text, confidence) = ctc_greedy_decode(&probabilities, 2, 3, &characters);
        assert!(text.is_empty());
        assert_eq!(confidence, 0.0);
    }

    #[test]
    fn dictionary_is_read_line_by_line() {
        let directory = std::env::temp_dir().join("lucid-spell-ocr-dict-test");
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("dict.txt");
        std::fs::write(&path, "a\nb\r\n \nc\n").unwrap();
        let dictionary = read_dictionary(&path).unwrap();
        assert_eq!(dictionary, vec!["a", "b", " ", "c"]);
    }
}
