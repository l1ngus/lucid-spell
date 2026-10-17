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

//! DB (Differentiable Binarization) post-processing: turns the probability map
//! of the detection model into text boxes.
//!
//! This follows PaddleOCR's / RapidOCR's `DBPostProcess`: threshold the map,
//! dilate it, find connected components, turn each component into its
//! minimal-area rectangle, score it, expand it ("unclip") and finally sort all
//! boxes for reading.

use crate::ocr::geometry::{
    convex_hull, min_area_rect, offset_polygon_round, order_quad_clockwise, polygon_mean,
    quad_area_perimeter, quad_side_lengths, Point,
};
use std::cmp::Ordering;

/// Threshold for binarizing the probability map.
const DET_THRESH: f32 = 0.3;
/// Minimum average score of a box to be kept.
const BOX_THRESH: f32 = 0.5;
/// How far boxes are expanded before cropping, relative to `area / perimeter`.
const UNCLIP_RATIO: f32 = 1.6;
/// Minimum side length of a detected box, in pixels.
const MIN_SIZE: f32 = 3.0;
/// Maximum number of candidate components, same as PaddleOCR.
const MAX_CANDIDATES: usize = 1000;
/// Vertical tolerance (in pixels) for the reading order.
const SORT_TOLERANCE: f32 = 10.0;

/// Converts a detection probability map into text boxes in the coordinate
/// space of the original image. `probability_map` is the raw `1x1xHxW` model
/// output, `width`/`height` are its spatial size and the image dimensions
/// describe the image that was fed into the model after scaling.
pub fn boxes_from_bitmap(
    probability_map: &[f32],
    width: usize,
    height: usize,
    original_width: u32,
    original_height: u32,
) -> Vec<[Point; 4]> {
    if width == 0 || height == 0 || probability_map.len() < width * height {
        return Vec::new();
    }

    let mut bitmap: Vec<bool> = probability_map[..width * height]
        .iter()
        .map(|value| *value > DET_THRESH)
        .collect();
    dilate_2x2(&mut bitmap, width, height);

    let mut visited = vec![false; width * height];
    let mut boxes = Vec::new();
    let mut candidates = 0usize;
    for start in 0..width * height {
        if candidates >= MAX_CANDIDATES {
            break;
        }
        if !bitmap[start] || visited[start] {
            continue;
        }
        candidates += 1;

        let component = collect_component(&bitmap, &mut visited, width, height, start);
        if let Some(quad) = box_from_component(probability_map, width, height, &component) {
            boxes.push(quad);
        }
    }

    let scale_x = original_width as f32 / width as f32;
    let scale_y = original_height as f32 / height as f32;
    let boxes = boxes
        .into_iter()
        .map(|quad| {
            quad.map(|point| {
                Point::new(
                    (point.x * scale_x)
                        .round()
                        .clamp(0.0, original_width as f32),
                    (point.y * scale_y)
                        .round()
                        .clamp(0.0, original_height as f32),
                )
            })
        })
        .collect();

    sort_boxes(boxes)
}

/// Binary dilation with a 2x2 kernel. With OpenCV's default anchor the output
/// pixel takes the maximum of the 2x2 block ending at that pixel.
fn dilate_2x2(mask: &mut [bool], width: usize, height: usize) {
    let source = mask.to_vec();
    for y in 0..height {
        for x in 0..width {
            let mut value = source[y * width + x];
            if x > 0 {
                value |= source[y * width + x - 1];
            }
            if y > 0 {
                value |= source[(y - 1) * width + x];
                if x > 0 {
                    value |= source[(y - 1) * width + x - 1];
                }
            }
            mask[y * width + x] = value;
        }
    }
}

/// Collects the pixels of one 8-connected component with a flood fill.
fn collect_component(
    mask: &[bool],
    visited: &mut [bool],
    width: usize,
    height: usize,
    start: usize,
) -> Vec<Point> {
    let mut pixels = Vec::new();
    let mut stack = vec![start];
    visited[start] = true;
    while let Some(index) = stack.pop() {
        let x = index % width;
        let y = index / width;
        pixels.push(Point::new(x as f32, y as f32));

        for dy in -1i64..=1 {
            for dx in -1i64..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let neighbor_x = x as i64 + dx;
                let neighbor_y = y as i64 + dy;
                if neighbor_x < 0
                    || neighbor_y < 0
                    || neighbor_x >= width as i64
                    || neighbor_y >= height as i64
                {
                    continue;
                }
                let neighbor = neighbor_y as usize * width + neighbor_x as usize;
                if mask[neighbor] && !visited[neighbor] {
                    visited[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }
    }
    pixels
}

/// Turns one connected component into a scored, expanded text box.
fn box_from_component(
    probabilities: &[f32],
    width: usize,
    height: usize,
    component: &[Point],
) -> Option<[Point; 4]> {
    let hull = convex_hull(component);
    let quad = order_quad_clockwise(min_area_rect(&hull)?);
    let (_, side) = quad_side_lengths(quad);
    if side < MIN_SIZE {
        return None;
    }

    let score = polygon_mean(probabilities, width, height, &quad);
    if score < BOX_THRESH {
        return None;
    }

    let (area, perimeter) = quad_area_perimeter(quad);
    if perimeter <= 0.0 {
        return None;
    }
    let delta = area * UNCLIP_RATIO / perimeter;
    let expanded_hull = convex_hull(&offset_polygon_round(&quad, delta));
    let expanded = order_quad_clockwise(min_area_rect(&expanded_hull)?);
    let (_, expanded_side) = quad_side_lengths(expanded);
    if expanded_side < MIN_SIZE + 2.0 {
        return None;
    }

    Some(expanded)
}

/// PaddleOCR's reading order: top to bottom, then left to right, with a
/// tolerance that keeps boxes of the same row ordered horizontally even if
/// their top edges are a few pixels apart.
fn sort_boxes(mut boxes: Vec<[Point; 4]>) -> Vec<[Point; 4]> {
    boxes.sort_by(|a, b| {
        a[0].y
            .partial_cmp(&b[0].y)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a[0].x.partial_cmp(&b[0].x).unwrap_or(Ordering::Equal))
    });

    for i in 0..boxes.len().saturating_sub(1) {
        let mut j = i as i64;
        while j >= 0 {
            let current = boxes[j as usize];
            let next = boxes[j as usize + 1];
            if (next[0].y - current[0].y).abs() < SORT_TOLERANCE && next[0].x < current[0].x {
                boxes.swap(j as usize, j as usize + 1);
                j -= 1;
            } else {
                break;
            }
        }
    }

    boxes
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Probability map with bright rectangles: `(x_start, y_start, x_end, y_end)`.
    fn probability_map(size: usize, rectangles: &[(usize, usize, usize, usize)]) -> Vec<f32> {
        let mut map = vec![0.05f32; size * size];
        for &(x_start, y_start, x_end, y_end) in rectangles {
            for y in y_start..y_end {
                for x in x_start..x_end {
                    map[y * size + x] = 0.9;
                }
            }
        }
        map
    }

    fn quad_center(quad: &[Point; 4]) -> (f32, f32) {
        let x = quad.iter().map(|p| p.x).sum::<f32>() / 4.0;
        let y = quad.iter().map(|p| p.y).sum::<f32>() / 4.0;
        (x, y)
    }

    #[test]
    fn finds_a_single_expanded_box() {
        let size = 64;
        let map = probability_map(size, &[(5, 10, 55, 22)]);
        let boxes = boxes_from_bitmap(&map, size, size, size as u32, size as u32);
        assert_eq!(boxes.len(), 1);

        let quad = boxes[0];
        let min_x = quad.iter().map(|p| p.x).fold(f32::MAX, f32::min);
        let max_x = quad.iter().map(|p| p.x).fold(f32::MIN, f32::max);
        let min_y = quad.iter().map(|p| p.y).fold(f32::MAX, f32::min);
        let max_y = quad.iter().map(|p| p.y).fold(f32::MIN, f32::max);
        // The box is expanded beyond the bright rectangle by the unclip step.
        assert!(min_x < 5.0, "min_x = {min_x}");
        assert!(max_x > 55.0, "max_x = {max_x}");
        assert!(min_y < 10.0, "min_y = {min_y}");
        assert!(max_y > 22.0, "max_y = {max_y}");
        // ...but not too far, the expansion is around 8 px for this rectangle.
        assert!(min_x > -6.0 && max_x < 66.0);
        assert!(min_y > 0.0 && max_y < 34.0);
    }

    #[test]
    fn ignores_small_specks() {
        let size = 32;
        let map = probability_map(size, &[(10, 10, 12, 12)]);
        let boxes = boxes_from_bitmap(&map, size, size, size as u32, size as u32);
        assert!(boxes.is_empty());
    }

    #[test]
    fn orders_boxes_top_to_bottom() {
        let size = 64;
        let map = probability_map(size, &[(5, 40, 55, 50), (5, 5, 55, 15), (5, 22, 55, 33)]);
        let boxes = boxes_from_bitmap(&map, size, size, size as u32, size as u32);
        assert_eq!(boxes.len(), 3);
        let centers: Vec<f32> = boxes.iter().map(|quad| quad_center(quad).1).collect();
        assert!(
            centers[0] < centers[1] && centers[1] < centers[2],
            "centers = {centers:?}"
        );
    }

    #[test]
    fn scales_boxes_to_the_original_image() {
        let size = 64;
        let map = probability_map(size, &[(10, 20, 40, 32)]);
        let boxes = boxes_from_bitmap(&map, size, size, 128, 128);
        assert_eq!(boxes.len(), 1);
        let (center_x, center_y) = quad_center(&boxes[0]);
        assert!((center_x - 50.0).abs() < 15.0, "center_x = {center_x}");
        assert!((center_y - 52.0).abs() < 15.0, "center_y = {center_y}");
    }

    #[test]
    fn sort_boxes_swaps_close_boxes_out_of_order() {
        let quad = |x: f32, y: f32| {
            [
                Point::new(x, y),
                Point::new(x + 10.0, y),
                Point::new(x + 10.0, y + 5.0),
                Point::new(x, y + 5.0),
            ]
        };
        let boxes = vec![quad(40.0, 0.0), quad(0.0, 4.0), quad(0.0, 20.0)];
        let sorted = sort_boxes(boxes);
        assert_eq!(sorted[0][0].x, 0.0);
        assert_eq!(sorted[1][0].x, 40.0);
        assert_eq!(sorted[2][0].y, 20.0);
    }
}
