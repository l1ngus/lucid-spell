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

//! Text detection model (PP-OCRv4 DB) on ONNX Runtime.

use crate::ocr::create_session;
use crate::ocr::db_postprocess::boxes_from_bitmap;
use crate::ocr::error::{ort_err, OcrError};
use crate::ocr::geometry::Point;
use crate::ocr::preprocess::prepare_det_input;
use image::RgbImage;
use ort::environment::Environment;
use ort::session::Session;
use ort::value::Tensor;
use std::path::Path;

/// The smaller side of the detection model input is scaled down to this value
/// (and both sides are then rounded to multiples of 32).
const LIMIT_SIDE_LEN: u32 = 736;

/// PP-OCRv4 text detection (DB) session.
pub struct DetModel {
    session: Session,
}

impl DetModel {
    pub fn load(path: &Path, environment: &Environment) -> Result<Self, OcrError> {
        Ok(Self {
            session: create_session(path, environment)?,
        })
    }

    /// Runs text detection and returns text quadrilaterals in the coordinate
    /// space of the original image, ordered for reading.
    pub fn detect(&mut self, image: &RgbImage) -> Result<Vec<[Point; 4]>, OcrError> {
        let input = prepare_det_input(image, LIMIT_SIDE_LEN);
        let (input_height, input_width) = (input.height, input.width);
        let tensor = Tensor::from_array(([1usize, 3, input_height, input_width], input.tensor))
            .map_err(ort_err)?;

        let outputs = self.session.run(ort::inputs![tensor]).map_err(ort_err)?;
        let (shape, probability_map) = outputs[0].try_extract_tensor::<f32>().map_err(ort_err)?;

        if shape.len() != 4 {
            return Err(OcrError::Ort(format!(
                "Unexpected detection model output shape: {shape:?}"
            )));
        }
        let height = shape[2] as usize;
        let width = shape[3] as usize;
        let (original_width, original_height) = image.dimensions();

        Ok(boxes_from_bitmap(
            probability_map,
            width,
            height,
            original_width,
            original_height,
        ))
    }
}
