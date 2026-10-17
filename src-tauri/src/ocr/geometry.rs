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

//! Geometry helpers for the detection pipeline: convex hulls, minimal-area
//! rectangles, polygon offsetting ("unclip") and perspective cropping.
//!
//! All functions work in image coordinates (x to the right, y downwards).

use image::{Rgb, RgbImage};
use std::cmp::Ordering;
use std::f32::consts::TAU;

/// A 2D point in image pixel coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance(self, other: Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// Cross product of `origin -> a` and `origin -> b`.
fn cross(origin: Point, a: Point, b: Point) -> f32 {
    (a.x - origin.x) * (b.y - origin.y) - (a.y - origin.y) * (b.x - origin.x)
}

/// Andrew's monotone chain convex hull. The returned points are ordered along
/// the hull (clockwise in image coordinates); collinear points are dropped.
pub fn convex_hull(points: &[Point]) -> Vec<Point> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    let mut sorted = points.to_vec();
    sorted.sort_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.y.partial_cmp(&b.y).unwrap_or(Ordering::Equal))
    });
    sorted.dedup_by(|a, b| a.x == b.x && a.y == b.y);
    if sorted.len() <= 2 {
        return sorted;
    }

    let mut hull: Vec<Point> = Vec::with_capacity(sorted.len() * 2);
    for &point in &sorted {
        while hull.len() >= 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], point) <= 0.0 {
            hull.pop();
        }
        hull.push(point);
    }
    let lower_len = hull.len() + 1;
    for &point in sorted.iter().rev().skip(1) {
        while hull.len() >= lower_len
            && cross(hull[hull.len() - 2], hull[hull.len() - 1], point) <= 0.0
        {
            hull.pop();
        }
        hull.push(point);
    }
    hull.pop();
    hull
}

/// Minimal-area rectangle around a convex hull (rotating calipers over the
/// hull edges). Returns `None` for degenerate hulls. Corner order is arbitrary;
/// use [`order_quad_clockwise`] to normalize it.
pub fn min_area_rect(hull: &[Point]) -> Option<[Point; 4]> {
    if hull.len() < 3 {
        return None;
    }

    let mut best_area = f32::MAX;
    let mut best_corners = [Point::new(0.0, 0.0); 4];

    for i in 0..hull.len() {
        let origin = hull[i];
        let next = hull[(i + 1) % hull.len()];
        let edge_x = next.x - origin.x;
        let edge_y = next.y - origin.y;
        let edge_len = (edge_x * edge_x + edge_y * edge_y).sqrt();
        if edge_len < 1e-6 {
            continue;
        }
        let axis_x = edge_x / edge_len;
        let axis_y = edge_y / edge_len;
        let normal_x = -axis_y;
        let normal_y = axis_x;

        let (mut min_axis, mut max_axis) = (f32::MAX, f32::MIN);
        let (mut min_normal, mut max_normal) = (f32::MAX, f32::MIN);
        for point in hull {
            let dx = point.x - origin.x;
            let dy = point.y - origin.y;
            let along = dx * axis_x + dy * axis_y;
            let across = dx * normal_x + dy * normal_y;
            min_axis = min_axis.min(along);
            max_axis = max_axis.max(along);
            min_normal = min_normal.min(across);
            max_normal = max_normal.max(across);
        }

        let area = (max_axis - min_axis) * (max_normal - min_normal);
        if area < best_area {
            best_area = area;
            let corner = |along: f32, across: f32| {
                Point::new(
                    origin.x + axis_x * along + normal_x * across,
                    origin.y + axis_y * along + normal_y * across,
                )
            };
            best_corners = [
                corner(min_axis, min_normal),
                corner(max_axis, min_normal),
                corner(max_axis, max_normal),
                corner(min_axis, max_normal),
            ];
        }
    }

    if best_area == f32::MAX {
        None
    } else {
        Some(best_corners)
    }
}

/// Orders four quadrilateral corners as top-left, top-right, bottom-right,
/// bottom-left (clockwise in image coordinates), matching PaddleOCR's
/// `get_mini_boxes`.
pub fn order_quad_clockwise(mut points: [Point; 4]) -> [Point; 4] {
    points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal));
    let (top_left, bottom_left) = if points[1].y > points[0].y {
        (0, 1)
    } else {
        (1, 0)
    };
    let (top_right, bottom_right) = if points[3].y > points[2].y {
        (2, 3)
    } else {
        (3, 2)
    };
    [
        points[top_left],
        points[top_right],
        points[bottom_right],
        points[bottom_left],
    ]
}

/// Width and height of a quadrilateral ordered by [`order_quad_clockwise`]:
/// the longest of the two opposite sides for each dimension.
pub fn quad_side_lengths(quad: [Point; 4]) -> (f32, f32) {
    let width = quad[0].distance(quad[1]).max(quad[2].distance(quad[3]));
    let height = quad[0].distance(quad[3]).max(quad[1].distance(quad[2]));
    (width, height)
}

/// Area and perimeter of a quadrilateral.
pub fn quad_area_perimeter(quad: [Point; 4]) -> (f32, f32) {
    let mut doubled_area = 0.0;
    let mut perimeter = 0.0;
    for i in 0..4 {
        let a = quad[i];
        let b = quad[(i + 1) % 4];
        doubled_area += a.x * b.y - b.x * a.y;
        perimeter += a.distance(b);
    }
    ((doubled_area / 2.0).abs(), perimeter)
}

/// Outward unit normal of the edge `from -> to`, given the orientation sign of
/// the polygon (positive signed area means clockwise in image coordinates).
fn outward_normal(from: Point, to: Point, orientation: f32) -> Point {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 1e-6 {
        Point::new(0.0, 0.0)
    } else if orientation > 0.0 {
        Point::new(dy / len, -dx / len)
    } else {
        Point::new(-dy / len, dx / len)
    }
}

/// Offsets a (convex) polygon outwards by `delta` with rounded joins, which is
/// the Minkowski sum of the polygon and a disc of radius `delta`. This matches
/// how PaddleOCR's `unclip` expands detection boxes.
pub fn offset_polygon_round(polygon: &[Point], delta: f32) -> Vec<Point> {
    const ARC_STEPS: usize = 16;

    let count = polygon.len();
    if count < 3 || delta <= 0.0 {
        return polygon.to_vec();
    }

    let signed_area: f32 = (0..count)
        .map(|i| {
            let a = polygon[i];
            let b = polygon[(i + 1) % count];
            a.x * b.y - b.x * a.y
        })
        .sum::<f32>()
        / 2.0;
    let orientation = if signed_area >= 0.0 { 1.0 } else { -1.0 };

    let mut offset = Vec::with_capacity(count * (ARC_STEPS + 1));
    for i in 0..count {
        let previous = polygon[(i + count - 1) % count];
        let current = polygon[i];
        let next = polygon[(i + 1) % count];

        let before = outward_normal(previous, current, orientation);
        let after = outward_normal(current, next, orientation);
        let start = before.y.atan2(before.x);
        let end = after.y.atan2(after.x);

        let mut sweep = end - start;
        if orientation > 0.0 {
            while sweep < 0.0 {
                sweep += TAU;
            }
        } else {
            while sweep > 0.0 {
                sweep -= TAU;
            }
        }

        for step in 0..=ARC_STEPS {
            let angle = start + sweep * (step as f32 / ARC_STEPS as f32);
            offset.push(Point::new(
                current.x + delta * angle.cos(),
                current.y + delta * angle.sin(),
            ));
        }
    }
    offset
}

/// Mean value of `bitmap` over the pixels inside `polygon` (used for detection
/// box scores, where `bitmap` is the probability map).
pub fn polygon_mean(bitmap: &[f32], width: usize, height: usize, polygon: &[Point]) -> f32 {
    if polygon.len() < 3 || width == 0 || height == 0 || bitmap.len() < width * height {
        return 0.0;
    }

    let (mut min_x, mut max_x) = (f32::MAX, f32::MIN);
    let (mut min_y, mut max_y) = (f32::MAX, f32::MIN);
    for point in polygon {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
    }
    let x_start = min_x.floor().max(0.0) as usize;
    let x_end = (max_x.ceil().max(0.0) as usize).min(width - 1);
    let y_start = min_y.floor().max(0.0) as usize;
    let y_end = (max_y.ceil().max(0.0) as usize).min(height - 1);
    if x_start > x_end || y_start > y_end {
        return 0.0;
    }

    let count = polygon.len();
    let mut sum = 0.0f64;
    let mut samples = 0u64;
    for y in y_start..=y_end {
        let sample_y = y as f32;
        for x in x_start..=x_end {
            let sample_x = x as f32;
            let mut inside = false;
            let mut j = count - 1;
            for i in 0..count {
                let a = polygon[i];
                let b = polygon[j];
                if (a.y > sample_y) != (b.y > sample_y) {
                    let t = (sample_y - a.y) / (b.y - a.y);
                    if sample_x < a.x + t * (b.x - a.x) {
                        inside = !inside;
                    }
                }
                j = i;
            }
            if inside {
                sum += bitmap[y * width + x] as f64;
                samples += 1;
            }
        }
    }

    if samples == 0 {
        0.0
    } else {
        (sum / samples as f64) as f32
    }
}

/// Homography mapping `from` onto `to` (both as top-left, top-right,
/// bottom-right, bottom-left), or `None` if the configuration is degenerate.
pub fn find_homography(from: [Point; 4], to: [Point; 4]) -> Option<[[f32; 3]; 3]> {
    let mut matrix = [[0f64; 9]; 8];
    for i in 0..4 {
        let (x, y) = (from[i].x as f64, from[i].y as f64);
        let (u, v) = (to[i].x as f64, to[i].y as f64);
        matrix[i * 2] = [x, y, 1.0, 0.0, 0.0, 0.0, -x * u, -y * u, u];
        matrix[i * 2 + 1] = [0.0, 0.0, 0.0, x, y, 1.0, -x * v, -y * v, v];
    }

    // Gaussian elimination with partial pivoting.
    for column in 0..8 {
        let mut pivot = column;
        for row in column + 1..8 {
            if matrix[row][column].abs() > matrix[pivot][column].abs() {
                pivot = row;
            }
        }
        if matrix[pivot][column].abs() < 1e-12 {
            return None;
        }
        matrix.swap(column, pivot);

        let inverse = 1.0 / matrix[column][column];
        for value in &mut matrix[column][column..] {
            *value *= inverse;
        }
        // Eliminate `column` from every other row. Splitting the matrix into
        // three disjoint parts lets the pivot row be read while the rows above
        // and below it are updated.
        let (rows_above, pivot_and_below) = matrix.split_at_mut(column);
        let (pivot_row, rows_below) = pivot_and_below.split_at_mut(1);
        let pivot = &pivot_row[0];
        for row in rows_above.iter_mut().chain(rows_below.iter_mut()) {
            if row[column] != 0.0 {
                let factor = row[column];
                for (value, pivot_value) in row[column..].iter_mut().zip(&pivot[column..]) {
                    *value -= factor * pivot_value;
                }
            }
        }
    }

    let solution = |row: usize| matrix[row][8] as f32;
    Some([
        [solution(0), solution(1), solution(2)],
        [solution(3), solution(4), solution(5)],
        [solution(6), solution(7), 1.0],
    ])
}

/// Samples the image at fractional coordinates with bilinear interpolation.
/// Positions outside the image are clamped to the border.
fn sample_bilinear(image: &RgbImage, x: f32, y: f32) -> Rgb<u8> {
    let (width, height) = image.dimensions();
    let x = x.clamp(0.0, width as f32 - 1.0);
    let y = y.clamp(0.0, height as f32 - 1.0);
    let x0 = x.floor() as u32;
    let y0 = y.floor() as u32;
    let x1 = (x0 + 1).min(width - 1);
    let y1 = (y0 + 1).min(height - 1);
    let fx = x - x0 as f32;
    let fy = y - y0 as f32;

    let top_left = image.get_pixel(x0, y0);
    let top_right = image.get_pixel(x1, y0);
    let bottom_left = image.get_pixel(x0, y1);
    let bottom_right = image.get_pixel(x1, y1);

    let mut result = Rgb([0u8; 3]);
    for channel in 0..3 {
        let top = top_left[channel] as f32 * (1.0 - fx) + top_right[channel] as f32 * fx;
        let bottom = bottom_left[channel] as f32 * (1.0 - fx) + bottom_right[channel] as f32 * fx;
        result[channel] = (top * (1.0 - fy) + bottom * fy).round().clamp(0.0, 255.0) as u8;
    }
    result
}

/// Rotates the image 90 degrees counter-clockwise (NumPy's `rot90`).
pub fn rotate90_ccw(image: &RgbImage) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut rotated = RgbImage::new(height, width);
    for y in 0..height {
        for x in 0..width {
            rotated.put_pixel(y, width - 1 - x, *image.get_pixel(x, y));
        }
    }
    rotated
}

/// Crops a text quadrilateral out of the image and rectifies it into an
/// axis-aligned rectangle. Regions that are taller than they are wide
/// (`height >= 1.5 * width`) are additionally rotated 90 degrees
/// counter-clockwise; the second element of the result tells whether that
/// happened. Matches PaddleOCR's `get_rotate_crop_image`.
pub fn crop_quad(image: &RgbImage, quad: [Point; 4]) -> Option<(RgbImage, bool)> {
    let (crop_width, crop_height) = quad_side_lengths(quad);
    if crop_width < 2.0 || crop_height < 2.0 {
        return None;
    }
    let width = crop_width.round() as u32;
    let height = crop_height.round() as u32;

    let destination = [
        Point::new(0.0, 0.0),
        Point::new(width as f32, 0.0),
        Point::new(width as f32, height as f32),
        Point::new(0.0, height as f32),
    ];
    // Backward mapping: for every destination pixel look up the source point.
    let transform = find_homography(destination, quad)?;

    let mut crop = RgbImage::new(width, height);
    for y in 0..height {
        let position_y = y as f32;
        for x in 0..width {
            let position_x = x as f32;
            let denominator =
                transform[2][0] * position_x + transform[2][1] * position_y + transform[2][2];
            if denominator.abs() < 1e-9 {
                continue;
            }
            let source_x =
                (transform[0][0] * position_x + transform[0][1] * position_y + transform[0][2])
                    / denominator;
            let source_y =
                (transform[1][0] * position_x + transform[1][1] * position_y + transform[1][2])
                    / denominator;
            crop.put_pixel(x, y, sample_bilinear(image, source_x, source_y));
        }
    }

    if height as f32 / width as f32 >= 1.5 {
        Some((rotate90_ccw(&crop), true))
    } else {
        Some((crop, false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgb;

    #[test]
    fn hull_of_square_with_inner_point_has_four_corners() {
        let points = [
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 10.0),
            Point::new(0.0, 10.0),
            Point::new(5.0, 5.0),
        ];
        let hull = convex_hull(&points);
        assert_eq!(hull.len(), 4);
    }

    #[test]
    fn hull_of_collinear_points_is_a_segment() {
        let points = [
            Point::new(0.0, 0.0),
            Point::new(5.0, 0.0),
            Point::new(10.0, 0.0),
        ];
        let hull = convex_hull(&points);
        assert!(hull.len() <= 2, "hull = {hull:?}");
        assert!(min_area_rect(&hull).is_none());
    }

    #[test]
    fn min_area_rect_of_rotated_rectangle_matches_area() {
        // 20x10 rectangle rotated by 45 degrees.
        let (cos, sin) = (0.5f32.sqrt(), 0.5f32.sqrt());
        let corners = [
            Point::new(0.0, 0.0),
            Point::new(20.0, 0.0),
            Point::new(20.0, 10.0),
            Point::new(0.0, 10.0),
        ]
        .map(|point| Point::new(point.x * cos - point.y * sin, point.x * sin + point.y * cos));
        let hull = convex_hull(&corners);
        let rect = order_quad_clockwise(min_area_rect(&hull).unwrap());
        let (width, height) = quad_side_lengths(rect);
        assert!((width - 20.0).abs() < 0.01, "width = {width}");
        assert!((height - 10.0).abs() < 0.01, "height = {height}");
        let (area, perimeter) = quad_area_perimeter(rect);
        assert!((area - 200.0).abs() < 0.1);
        assert!((perimeter - 60.0).abs() < 0.1);
    }

    #[test]
    fn orders_quad_clockwise() {
        let quad = [
            Point::new(10.0, 10.0),
            Point::new(0.0, 0.0),
            Point::new(0.0, 10.0),
            Point::new(10.0, 0.0),
        ];
        let ordered = order_quad_clockwise(quad);
        assert_eq!(ordered[0], Point::new(0.0, 0.0));
        assert_eq!(ordered[1], Point::new(10.0, 0.0));
        assert_eq!(ordered[2], Point::new(10.0, 10.0));
        assert_eq!(ordered[3], Point::new(0.0, 10.0));
    }

    #[test]
    fn offsetting_square_rounds_the_corners() {
        let square = [
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 10.0),
            Point::new(0.0, 10.0),
        ];
        let expanded = offset_polygon_round(&square, 2.0);
        let hull = convex_hull(&expanded);
        let rect = min_area_rect(&hull).unwrap();
        let min_x = rect.iter().map(|p| p.x).fold(f32::MAX, f32::min);
        let max_x = rect.iter().map(|p| p.x).fold(f32::MIN, f32::max);
        let min_y = rect.iter().map(|p| p.y).fold(f32::MAX, f32::min);
        let max_y = rect.iter().map(|p| p.y).fold(f32::MIN, f32::max);
        assert!((min_x + 2.0).abs() < 0.05, "min_x = {min_x}");
        assert!((max_x - 12.0).abs() < 0.05, "max_x = {max_x}");
        assert!((min_y + 2.0).abs() < 0.05, "min_y = {min_y}");
        assert!((max_y - 12.0).abs() < 0.05, "max_y = {max_y}");
    }

    #[test]
    fn polygon_mean_ignores_outside_pixels() {
        // 3x3 bitmap where only the center pixel is bright.
        let bitmap = [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        let around_center = [
            Point::new(0.5, 0.5),
            Point::new(1.5, 0.5),
            Point::new(1.5, 1.5),
            Point::new(0.5, 1.5),
        ];
        assert!((polygon_mean(&bitmap, 3, 3, &around_center) - 1.0).abs() < 1e-6);

        let around_corner = [
            Point::new(-0.5, -0.5),
            Point::new(0.5, -0.5),
            Point::new(0.5, 0.5),
            Point::new(-0.5, 0.5),
        ];
        assert!((polygon_mean(&bitmap, 3, 3, &around_corner) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn crop_of_full_image_is_identity() {
        let mut image = RgbImage::new(16, 8);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            *pixel = Rgb([x as u8, y as u8, (x + y) as u8]);
        }
        let quad = [
            Point::new(0.0, 0.0),
            Point::new(16.0, 0.0),
            Point::new(16.0, 8.0),
            Point::new(0.0, 8.0),
        ];
        let (crop, rotated) = crop_quad(&image, quad).unwrap();
        assert!(!rotated);
        assert_eq!(crop.dimensions(), (16, 8));
        for (x, y, pixel) in image.enumerate_pixels() {
            assert_eq!(crop.get_pixel(x, y), pixel);
        }
    }

    #[test]
    fn vertical_crop_is_rotated() {
        let mut image = RgbImage::new(20, 4);
        for (_, y, pixel) in image.enumerate_pixels_mut() {
            *pixel = Rgb([y as u8 * 30, 0, 0]);
        }
        // A tall box: 3 px wide, 19 px high.
        let quad = [
            Point::new(0.0, 0.0),
            Point::new(3.0, 0.0),
            Point::new(3.0, 19.0),
            Point::new(0.0, 19.0),
        ];
        let (crop, rotated) = crop_quad(&image, quad).unwrap();
        assert!(rotated);
        assert_eq!(crop.dimensions(), (19, 3));
    }

    #[test]
    fn rotate90_ccw_moves_first_column_to_last_row() {
        let mut image = RgbImage::new(3, 2);
        for (x, _, pixel) in image.enumerate_pixels_mut() {
            *pixel = Rgb([x as u8, 0, 0]);
        }
        let rotated = rotate90_ccw(&image);
        assert_eq!(rotated.dimensions(), (2, 3));
        // Input column 0 becomes the output row 2 (left to right).
        assert_eq!(rotated.get_pixel(0, 2), &Rgb([0, 0, 0]));
        assert_eq!(rotated.get_pixel(1, 2), &Rgb([0, 0, 0]));
        assert_eq!(rotated.get_pixel(0, 0), &Rgb([2, 0, 0]));
        assert_eq!(rotated.get_pixel(1, 0), &Rgb([2, 0, 0]));
    }
}
