// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Types to model film and pixels in the sensor of the simulated sensor.

use log::info;

use crate::core::filter::Filter;
use crate::core::geometry::Bounds2f;
use crate::core::geometry::Bounds2i;
use crate::core::geometry::Point2f;
use crate::core::geometry::Point2i;
use crate::core::geometry::Vector2f;
use crate::core::parallel::AtomicFloat;
use crate::Float;

const FILTER_TABLE_WIDTH: usize = 16;

struct Pixel {
    xyz: [Float; 3],
    filter_weight_sum: Float,
    splat_xyz: [AtomicFloat; 3],
    _pad: Float,
}

pub struct Film {
    pub full_resolution: Point2i,
    crop_window: Bounds2f,
    pub filter: Box<dyn Filter>,
    pub diagonal_m: Float,
    pub filename: String,
    scale: Float,
    pub cropped_pixel_bounds: Bounds2i,
    pixels: Vec<Pixel>,
    filter_table: Vec<Float>,
    max_sample_luminance: Float,
}

impl Film {
    pub fn new(
        resolution: Point2i,
        crop_window: Bounds2f,
        filter: Box<dyn Filter>,
        diagonal_mm: Float,
        filename: String,
        scale: Float,
        max_sample_luminance: Float,
    ) -> Film {
        let full_resolution = resolution;
        let cropped_pixel_bounds = Bounds2i::from((
            Point2i::from((
                (full_resolution.x as Float * crop_window.p_min.x).ceil() as isize,
                (full_resolution.y as Float * crop_window.p_min.y).ceil() as isize,
            )),
            Point2i::from((
                (full_resolution.x as Float * crop_window.p_max.x).ceil() as isize,
                (full_resolution.y as Float * crop_window.p_max.y).ceil() as isize,
            )),
        ));
        info!(
            "Created film with full resolution {}. Crop window of {} -> croppedPixelBounds {}",
            resolution, crop_window, cropped_pixel_bounds
        );
        let pixels = Vec::with_capacity(cropped_pixel_bounds.area() as usize);
        // TODO(wathiede): increment global stats like:
        // filmPixelMemory += croppedPixelBounds.Area() * sizeof(Pixel);
        let w = FILTER_TABLE_WIDTH as Float;
        // Precompute filter weight table
        let mut filter_table = Vec::with_capacity(FILTER_TABLE_WIDTH * FILTER_TABLE_WIDTH);
        for y in 0..FILTER_TABLE_WIDTH {
            for x in 0..FILTER_TABLE_WIDTH {
                filter_table.push(filter.evaluate(Point2f {
                    x: (x as Float + 0.5) * filter.radius().x / w,
                    y: (y as Float + 0.5) * filter.radius().y / w,
                }))
            }
        }

        Film {
            full_resolution,
            crop_window,
            filter,
            diagonal_m: diagonal_mm * 0.001,
            filename,
            cropped_pixel_bounds,
            pixels,
            filter_table,
            scale,
            max_sample_luminance,
        }
    }

    /// Return the bounding box for sampling this `Film`.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::film::Film;
    /// use pbrt::core::geometry::Bounds2i;
    /// use pbrt::core::geometry::Point2i;
    /// use pbrt::filters::boxfilter::BoxFilter;
    ///
    /// let filter = BoxFilter::new([8., 8.].into());
    /// let film = Film::new(
    ///     [1920, 1080].into(),
    ///     ([0.25, 0.25], [0.75, 0.75]).into(),
    ///     Box::new(filter),
    ///     35.0,
    ///     "output.png".to_string(),
    ///     1.,
    ///     1.,
    /// );
    /// assert_eq!(
    ///     film.get_sample_bounds(),
    ///     Bounds2i::from(([472, 262], [1448, 818]))
    /// );
    /// ```
    pub fn get_sample_bounds(&self) -> Bounds2i {
        Bounds2f::from((
            (Point2f::from(self.cropped_pixel_bounds.p_min) + Vector2f::from([0.5, 0.5])
                - self.filter.radius())
            .floor(),
            (Point2f::from(self.cropped_pixel_bounds.p_max) - Vector2f::from([0.5, 0.5])
                + self.filter.radius())
            .ceil(),
        ))
        .into()
    }
}
