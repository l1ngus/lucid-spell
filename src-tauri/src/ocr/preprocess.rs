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

//! Image preprocessing for the PaddleOCR detection and recognition models.
//!
//! The constants and the resize/normalize logic follow RapidOCR's
//! `DBProcess` (detection) and `CRNNProcess` (recognition), which mirror the
//! official PaddleOCR preprocessing.

use image::imageops::FilterType;
use image::{imageops, RgbImage};

/// Normalization constants of ImageNet, used by the detection model.
const DET_MEAN: [f32; 3] = [0.485, 0.456, 0.406];
const DET_STD: [f32; 3] = [0.229, 0.224, 0.225];

/// Smallest input width of the recognition model.
const REC_MIN_WIDTH: usize = 320;

/// Preprocessed detection model input: a normalized `1x3xHxW` NCHW tensor.
pub struct DetInput {
    pub tensor: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

/// Preprocessed recognition model input: a normalized `1x3x48xW` NCHW tensor.
pub struct RecInput {
    pub tensor: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

/// Prepares an image for the detection model: the smaller side is scaled down
/// to `limit_side_len` (if it is larger), both sides are rounded to multiples
/// of 32 and the result is normalized into an NCHW tensor.
pub fn prepare_det_input(image: &RgbImage, limit_side_len: u32) -> DetInput {
    let (original_width, original_height) = image.dimensions();
    let ratio = limit_side_len as f32 / original_width.min(original_height).max(1) as f32;
    let (mut width, mut height) = if ratio < 1.0 {
        (
            (original_width as f32 * ratio) as usize,
            (original_height as f32 * ratio) as usize,
        )
    } else {
        (original_width as usize, original_height as usize)
    };
    width = ((width as f32 / 32.0).round() as usize * 32).max(32);
    height = ((height as f32 / 32.0).round() as usize * 32).max(32);

    let resized = imageops::resize(image, width as u32, height as u32, FilterType::Triangle);
    let plane = width * height;
    let mut tensor = vec![0.0f32; 3 * plane];
    for (y, row) in resized.rows().enumerate() {
        for (x, pixel) in row.enumerate() {
            let index = y * width + x;
            for channel in 0..3 {
                tensor[channel * plane + index] =
                    (pixel[channel] as f32 / 255.0 - DET_MEAN[channel]) / DET_STD[channel];
            }
        }
    }

    DetInput {
        tensor,
        width,
        height,
    }
}

/// Prepares a cropped text line for the recognition model: the crop is scaled
/// to the model height (keeping the aspect ratio, but never taking more width
/// than the full line would need) and padded with zeros on the right, matching
/// PaddleOCR's `resize_norm_img`.
pub fn prepare_rec_input(image: &RgbImage, height: usize) -> RecInput {
    let (original_width, original_height) = image.dimensions();
    let ratio = original_width.max(1) as f32 / original_height.max(1) as f32;
    let full_width = (height as f32 * ratio) as usize;
    let target_width = full_width.max(REC_MIN_WIDTH);
    let resized_width = ((height as f32 * ratio).ceil() as usize)
        .max(1)
        .min(target_width);

    let resized = imageops::resize(
        image,
        resized_width as u32,
        height as u32,
        FilterType::Triangle,
    );
    let plane = target_width * height;
    let mut tensor = vec![0.0f32; 3 * plane];
    for (y, row) in resized.rows().enumerate() {
        for (x, pixel) in row.enumerate() {
            let index = y * target_width + x;
            for channel in 0..3 {
                tensor[channel * plane + index] = (pixel[channel] as f32 / 255.0 - 0.5) / 0.5;
            }
        }
    }

    RecInput {
        tensor,
        width: target_width,
        height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgb;

    fn gradient_image(width: u32, height: u32) -> RgbImage {
        let mut image = RgbImage::new(width, height);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            *pixel = Rgb([(x % 256) as u8, (y % 256) as u8, 128]);
        }
        image
    }

    #[test]
    fn det_input_is_scaled_to_multiples_of_32() {
        let image = gradient_image(1472, 920);
        let input = prepare_det_input(&image, 736);
        // The smaller side is scaled down to ~736 and both sides are rounded
        // to multiples of 32.
        assert_eq!(input.width % 32, 0);
        assert_eq!(input.height % 32, 0);
        assert!(input.height <= 736 + 32);
        assert_eq!(input.tensor.len(), 3 * input.width * input.height);
    }

    #[test]
    fn small_det_input_is_not_upscaled() {
        let image = gradient_image(320, 64);
        let input = prepare_det_input(&image, 736);
        assert_eq!((input.width, input.height), (320, 64));
    }

    #[test]
    fn rec_input_is_padded_to_min_width() {
        let crop = gradient_image(10, 10);
        let input = prepare_rec_input(&crop, 48);
        assert_eq!(input.height, 48);
        assert_eq!(input.width, REC_MIN_WIDTH);

        // All three channels are normalized into [-1, 1].
        for value in &input.tensor {
            assert!((-1.0..=1.0).contains(value));
        }
        // The crop occupies the left 48 columns, everything to the right is
        // zero padding.
        let plane = input.width * input.height;
        for channel in 0..3 {
            for y in 0..48 {
                for x in 48..input.width {
                    assert_eq!(input.tensor[channel * plane + y * input.width + x], 0.0);
                }
            }
        }
    }

    #[test]
    fn rec_input_keeps_long_lines_wide() {
        let crop = gradient_image(481, 48);
        let input = prepare_rec_input(&crop, 48);
        // `int(48 * ratio)` can be off by one because of float rounding, both
        // in this implementation and in the original PaddleOCR code.
        assert!(
            (480..=481).contains(&input.width),
            "width = {}",
            input.width
        );
        assert_eq!(input.tensor.len(), 3 * input.width * 48);
    }
}
