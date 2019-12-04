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
use crate::core::spectrum::Spectrum;
use crate::Float;

const FILTER_TABLE_WIDTH: usize = 16;

struct FilmTilePixel {
    contrib_sum: Spectrum,
    filter_weight_sum: Float,
}

struct Pixel {
    xyz: [Float; 3],
    filter_weight_sum: Float,
    splat_xyz: [AtomicFloat; 3],
    _pad: Float,
}

/// Film models the sensor on a simulated camera.  It may have a `crop_window` that limits
/// rendering to a subset of the `Film`.
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
    ///     [[0.25, 0.25], [0.75, 0.75]].into(),
    ///     Box::new(filter),
    ///     35.0,
    ///     "output.png".to_string(),
    ///     1.,
    ///     1.,
    /// );
    /// assert_eq!(
    ///     film.get_sample_bounds(),
    ///     Bounds2i::from([[472, 262], [1448, 818]])
    /// );
    /// ```
    pub fn get_sample_bounds(&self) -> Bounds2i {
        let half_pixel = Vector2f::from([0.5, 0.5]);
        Bounds2f::from([
            (Point2f::from(self.cropped_pixel_bounds.p_min) + half_pixel - self.filter.radius())
                .floor(),
            (Point2f::from(self.cropped_pixel_bounds.p_max) - half_pixel + self.filter.radius())
                .ceil(),
        ])
        .into()
    }

    /// Compute physical size of the film.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::film::Film;
    /// use pbrt::core::geometry::Bounds2f;
    /// use pbrt::filters::boxfilter::BoxFilter;
    ///
    /// let filter = BoxFilter::new([8., 8.].into());
    /// let diag_mm = 100.;
    /// let film = Film::new(
    ///     [800, 600].into(),
    ///     [[0., 0.], [1., 1.]].into(),
    ///     Box::new(filter),
    ///     diag_mm,
    ///     "output.png".to_string(),
    ///     1.,
    ///     1.,
    /// );
    /// assert_eq!(
    ///     film.get_physical_extent(),
    ///     Bounds2f::from([[-0.04, -0.03], [0.04, 0.03]])
    /// );
    ///
    /// let filter = BoxFilter::new([8., 8.].into());
    /// let film = Film::new(
    ///     [800, 600].into(),
    ///     // The result of get_physical_extent doesn't change if crop_window is a subset of the Film.
    ///     [[0.25, 0.25], [0.75, 0.75]].into(),
    ///     Box::new(filter),
    ///     diag_mm,
    ///     "output.png".to_string(),
    ///     1.,
    ///     1.,
    /// );
    /// assert_eq!(
    ///     film.get_physical_extent(),
    ///     Bounds2f::from([[-0.04, -0.03], [0.04, 0.03]])
    /// );
    /// ```
    pub fn get_physical_extent(&self) -> Bounds2f {
        let aspect = self.full_resolution.y as Float / self.full_resolution.x as Float;
        let x = (self.diagonal_m * self.diagonal_m / (1. + aspect * aspect)).sqrt();
        let y = aspect * x;
        [
            Point2f::from([-x / 2., -y / 2.]),
            Point2f::from([x / 2., y / 2.]),
        ]
        .into()
    }

    /// Create a `FilmTile` representing the subregion of this `Film` denoted by `sample_bounds`.
    /// The `FilmTile` should have its pixels contributed to the `Film` by calling
    /// `merge_film_tile`.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::film::Film;
    /// use pbrt::core::geometry::Bounds2i;
    /// use pbrt::filters::boxfilter::BoxFilter;
    ///
    /// let filter = BoxFilter::new([8., 8.].into());
    /// let film = Film::new(
    ///     [1920, 1080].into(),
    ///     [[0.25, 0.25], [0.75, 0.75]].into(),
    ///     Box::new(filter),
    ///     35.0,
    ///     "output.png".to_string(),
    ///     1.,
    ///     1.,
    /// );
    ///
    /// // Tile bigger than Film's crop area gets clipped to Film's crop area.
    /// assert_eq!(
    ///     film.get_film_tile(Bounds2i::from([[0, 0], [1920, 1080]]))
    ///         .get_pixel_bounds(),
    ///     Bounds2i::from([[1920 / 4, 1080 / 4], [3 * 1920 / 4, 3 * 1080 / 4]])
    /// );
    /// // Tile smaller than Film's crop area is the given bound expanded by half the filter size.
    /// assert_eq!(
    ///     film.get_film_tile(Bounds2i::from([[500, 500], [600, 600]]))
    ///         .get_pixel_bounds(),
    ///     Bounds2i::from([[492, 492], [608, 608]])
    /// );
    /// ```
    pub fn get_film_tile(&self, sample_bounds: Bounds2i) -> FilmTile {
        let half_pixel = Vector2f::from([0.5, 0.5]);
        let float_bounds = Bounds2f::from(sample_bounds);
        let p0 = Point2i::from((float_bounds.p_min - half_pixel - self.filter.radius()).ceil());
        let p1 = Point2i::from(
            (float_bounds.p_max - half_pixel + self.filter.radius()).floor()
                + Point2f::from([1., 1.]),
        );
        let tile_pixel_bounds =
            Bounds2i::intersect(&Bounds2i::from([p0, p1]), &self.cropped_pixel_bounds);
        FilmTile::new(
            tile_pixel_bounds,
            self.filter.radius(),
            &self.filter_table,
            FILTER_TABLE_WIDTH,
            self.max_sample_luminance,
        )
        /*
        return std::unique_ptr<FilmTile>(new FilmTile(
        tilePixelBounds, filter->radius, filterTable, filterTableWidth,
        maxSampleLuminance));
        */
    }

    pub fn merge_film_tile(&self, tile: FilmTile) {
        unimplemented!()
    }

    pub fn set_image(&self, img: Vec<Spectrum>) {
        unimplemented!()
    }

    pub fn add_splat(&self, p: &Point2f, v: Spectrum) {
        unimplemented!()
    }

    pub fn write_image(&self, splat_scale: Float) {
        unimplemented!()
    }

    pub fn clear(&self) {
        unimplemented!()
    }
}

/// FilmTile represents a subarea of `Film` within the `Film`'s configured sampling bounds.  It
/// allows rendering of portions of the `Film` to be handed off to separate threads, and the final
/// assembly of the full image is handled by passing the `FilmTile` back to the `Film` via
/// [merge_film_tile]
///
/// [merge_film_tile]: Film::merge_film_tile
pub struct FilmTile<'ft> {
    pixel_bounds: Bounds2i,
    filter_radius: Vector2f,
    inv_filter_radius: Vector2f,
    filter_table: &'ft Vec<Float>,
    filter_table_size: usize,
    max_sample_luminance: Float,
    pixels: Vec<FilmTilePixel>,
}

impl<'ft> FilmTile<'ft> {
    fn new(
        pixel_bounds: Bounds2i,
        filter_radius: Vector2f,
        filter_table: &'ft Vec<Float>,
        filter_table_size: usize,
        max_sample_luminance: Float,
    ) -> FilmTile<'ft> {
        FilmTile {
            pixel_bounds,
            filter_radius,
            inv_filter_radius: [1. / filter_radius.x, 1. / filter_radius.y].into(),
            filter_table,
            filter_table_size,
            pixels: Vec::new(),
            max_sample_luminance,
        }
    }

    /// Get the pixel bounds for this tile.  For example see [get_film_tile].
    ///
    /// [get_film_tile]: Film::get_film_tile
    pub fn get_pixel_bounds(&self) -> Bounds2i {
        self.pixel_bounds
    }
}
