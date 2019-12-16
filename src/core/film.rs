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
#![deny(missing_docs)]

//! Types to model film and pixels in the sensor of the simulated sensor.

use std::convert::TryInto;
use std::sync::Arc;
use std::sync::Mutex;

use log::info;

use crate::core::filter::Filter;
use crate::core::geometry::Bounds2f;
use crate::core::geometry::Bounds2i;
use crate::core::geometry::Point2f;
use crate::core::geometry::Point2i;
use crate::core::geometry::Vector2f;
use crate::core::imageio::write_image;
use crate::core::spectrum::xyz_to_rgb;
use crate::core::spectrum::Spectrum;
use crate::Float;

const FILTER_TABLE_WIDTH: usize = 16;

#[derive(Default)]
/// Pixel type for `FilmTile`, represents an intermediate pixel type before being merged back into
/// `Film`.
pub struct FilmTilePixel {
    contrib_sum: Spectrum,
    filter_weight_sum: Float,
}

#[derive(Debug)]
/// Top level pixel type for `Film`.
/// Not public in the C++ implementation, but necessary for docttest.
pub struct Pixel {
    xyz: [Float; 3],
    filter_weight_sum: Float,
    // TOOD(wathiede): make this AtomicFloat if that proves necessary.
    // splat_xyz: [AtomicFloat; 3],
    splat_xyz: [Float; 3],
    /* TODO(wathiede): figure how how to do this and if it is worth it to prevent unaligned struct.
     * _pad: Float, */
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel {
            xyz: Default::default(),
            filter_weight_sum: Default::default(),
            splat_xyz: Default::default(),
        }
    }
}

/// Film models the sensor on a simulated camera.  It may have a `crop_window` that limits
/// rendering to a subset of the `Film`.
pub struct Film {
    /// full_resolution represents the full extents of the film.  This `Film` may be further
    /// limited by `crop_window`.
    pub full_resolution: Point2i,
    crop_window: Bounds2f,
    /// filter specifies the sampling algorithm to use when evaluating pixels in the `Film`.
    pub filter: Box<dyn Filter>,
    /// physical distance of the `Film`'s diagonal in meters.
    pub diagonal_m: Float,
    /// filename to store the contents of the `Film`
    pub filename: String,
    scale: Float,
    /// cropped_pixel_bounds represents the portion of the `Film` to render
    pub cropped_pixel_bounds: Bounds2i,
    pixels: Arc<Mutex<Vec<Pixel>>>,
    filter_table: Vec<Float>,
    max_sample_luminance: Float,
}

impl Film {
    /// new creates a `Film` struct from the given parameters. Note that `diagonal_mm` specifies
    /// the physical diagonal size of the `Film` in millimeters, but the internal representation is
    /// meters.
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
        let pixels = Arc::new(Mutex::new(
            (0..cropped_pixel_bounds.area())
                .map(|_| Pixel::default())
                .collect(),
        ));
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
    }

    /// Merge a `FilmTile` into the `Film`.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::film::Film;
    /// use pbrt::core::film::FilmTile;
    /// use pbrt::core::film::Pixel;
    /// use pbrt::core::geometry::Bounds2i;
    /// use pbrt::core::spectrum::Spectrum;
    /// use pbrt::filters::boxfilter::BoxFilter;
    ///
    /// let filter = BoxFilter::new([8., 8.].into());
    /// let film = Film::new(
    ///     [20, 10].into(),
    ///     [[0., 0.], [1., 1.]].into(),
    ///     Box::new(filter),
    ///     35.0,
    ///     "output.png".to_string(),
    ///     1.,
    ///     1.,
    /// );
    ///
    /// let left = film.get_film_tile(Bounds2i::from([[0, 0], [10, 10]]));
    /// let right = film.get_film_tile(Bounds2i::from([[10, 0], [10, 10]]));
    /// // spawn threads and render to left and right.  Then merge the results back into the film.
    /// film.merge_film_tile(left);
    /// film.merge_film_tile(right);
    /// ```
    pub fn merge_film_tile(&self, tile: FilmTile) {
        // TODO(wathiede): ProfilePhase p(Prof::MergeFilmTile);
        info!("Merging film tile {}", tile.pixel_bounds);
        let mut pixels = self.pixels.lock().unwrap();
        for pixel in tile.get_pixel_bounds().iter() {
            let tile_pixel = tile.get_pixel(pixel);
            let merge_pixel = &mut pixels[self.pixel_offset(pixel)];
            let xyz = tile_pixel.contrib_sum.to_xyz();
            for i in 0..3 {
                merge_pixel.xyz[i] += xyz[i];
            }
            merge_pixel.filter_weight_sum += tile_pixel.filter_weight_sum;
        }
    }

    /// set_image allows the caller to directly set the pixel values of the entire `Film`
    pub fn set_image(&self, img: Vec<Spectrum>) {
        unimplemented!()
    }

    /// add_splat adds the contributions of `v` to the `Film` at `p`
    pub fn add_splat(&self, p: &Point2f, v: Spectrum) {
        unimplemented!()
    }

    /// write_image stores the contents of the `Film` to the disk path specifed at construction
    /// time.
    pub fn write_image(&self, splat_scale: Float) {
        info!("Converting image to RGB and computing final weighted pixel values");
        let mut rgb: Vec<Float> = (0..(3 * self.cropped_pixel_bounds.area() as usize))
            .map(|_| 0.)
            .collect();
        let mut offset = 0;
        let mut pixels = self.pixels.lock().unwrap();
        for p in self.cropped_pixel_bounds.iter() {
            let pixel = &mut pixels[self.pixel_offset(p)];
            let c = xyz_to_rgb(pixel.xyz);
            rgb[3 * offset + 0] = c[0];
            rgb[3 * offset + 1] = c[1];
            rgb[3 * offset + 2] = c[2];

            // Normalize pixel with weight sum
            let filter_weight_sum = pixel.filter_weight_sum;
            if filter_weight_sum != 0. {
                let inv_wt = 1. / filter_weight_sum;

                rgb[3 * offset + 0] = (rgb[3 * offset] * inv_wt).max(0.);
                rgb[3 * offset + 1] = (rgb[3 * offset + 1] * inv_wt).max(0.);
                rgb[3 * offset + 2] = (rgb[3 * offset + 2] * inv_wt).max(0.);
            }

            // Add splat value at pixel
            let splat_rgb = xyz_to_rgb(pixel.splat_xyz);
            rgb[3 * offset + 0] += splat_scale * splat_rgb[0];
            rgb[3 * offset + 1] += splat_scale * splat_rgb[1];
            rgb[3 * offset + 2] += splat_scale * splat_rgb[2];
            //Scale pixel value by `scale`
            rgb[3 * offset + 0] *= self.scale;
            rgb[3 * offset + 1] *= self.scale;
            rgb[3 * offset + 2] *= self.scale;

            offset += 1;
        }
        info!(
            "Writing image {} with bounds {}",
            self.filename, self.cropped_pixel_bounds
        );
        write_image(
            &self.filename,
            &rgb,
            self.cropped_pixel_bounds,
            self.full_resolution,
        );
    }

    /// clear resets all pixel values to zero.
    pub fn clear(&self) {
        unimplemented!()
    }

    fn pixel_offset(&self, p: Point2i) -> usize {
        debug_assert!(
            self.cropped_pixel_bounds.inside_exclusive(p),
            "p {} outside {}",
            p,
            self.cropped_pixel_bounds
        );
        let width = self.cropped_pixel_bounds.p_max.x - self.cropped_pixel_bounds.p_min.x;
        ((p.x - self.cropped_pixel_bounds.p_min.x)
            + (p.y - self.cropped_pixel_bounds.p_min.y) * width)
            .try_into()
            .unwrap()
    }

    /// Not public in the C++ implementation, but necessary for docttest.
    pub fn get_pixel_xyz(&self, p: Point2i) -> [Float; 3] {
        debug_assert!(self.cropped_pixel_bounds.inside_exclusive(p));
        let offset = self.pixel_offset(p);
        let pixels = self.pixels.lock().unwrap();
        pixels[offset].xyz
    }

    /*
    /// Not public in the C++ implementation, but necessary for docttest.
    pub fn get_pixel_mut(&mut self, p: Point2i) -> &mut Pixel {
        debug_assert!(inside_exclusive(p, self.cropped_pixel_bounds));
        let offset = self.pixel_offset(p);
        &mut self.pixels[offset]
    }
    */
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
        let pixel_count = 0.max(pixel_bounds.area());
        FilmTile {
            pixel_bounds,
            filter_radius,
            inv_filter_radius: [1. / filter_radius.x, 1. / filter_radius.y].into(),
            filter_table,
            filter_table_size,
            pixels: (0..pixel_count).map(|_| FilmTilePixel::default()).collect(),
            max_sample_luminance,
        }
    }

    /// Get the pixel bounds for this tile.  For example see [get_film_tile].
    ///
    /// [get_film_tile]: Film::get_film_tile
    pub fn get_pixel_bounds(&self) -> Bounds2i {
        self.pixel_bounds
    }

    fn pixel_offset(&self, p: Point2i) -> usize {
        debug_assert!(
            self.pixel_bounds.inside_exclusive(p),
            "p {} outside {}",
            p,
            self.pixel_bounds
        );
        let width = self.pixel_bounds.p_max.x - self.pixel_bounds.p_min.x;
        ((p.x - self.pixel_bounds.p_min.x) + (p.y - self.pixel_bounds.p_min.y) * width)
            .try_into()
            .unwrap()
    }

    /// get_pixel returns the `FilmTile` value at the given `p`
    pub fn get_pixel(&self, p: Point2i) -> &FilmTilePixel {
        let offset = self.pixel_offset(p);
        &self.pixels[offset]
    }

    /// get_pixel_mut returns a mutable `FilmTile` value at the given `p`
    pub fn get_pixel_mut(&mut self, p: Point2i) -> &mut FilmTilePixel {
        let offset = self.pixel_offset(p);
        &mut self.pixels[offset]
    }
}

#[cfg(test)]
mod test {
    use crate::core::film::Film;
    use crate::core::film::FilmTile;
    use crate::core::geometry::Bounds2i;
    use crate::core::spectrum::Spectrum;
    use crate::filters::boxfilter::BoxFilter;
    use crate::Float;

    #[test]
    fn merge_film_tile() {
        fn fill(t: &mut FilmTile, c: &Spectrum) {
            for pt in t.get_pixel_bounds().iter() {
                let px = t.get_pixel_mut(pt);
                px.contrib_sum = c.clone();
                px.filter_weight_sum = 1.;
            }
        }

        let filter = BoxFilter::new([8., 8.].into());
        let film = Film::new(
            [200, 10].into(),
            [[0., 0.], [1., 1.]].into(),
            Box::new(filter),
            35.0,
            "target/doc/pbrt/merge_film_tile.png".to_string(),
            1.,
            1.,
        );

        let mut left = film.get_film_tile(Bounds2i::from([[0, 0], [100, 10]]));
        let mut right = film.get_film_tile(Bounds2i::from([[100, 0], [200, 10]]));
        let green = Spectrum::from_rgb([0., 1., 0.]);
        let red = Spectrum::from_rgb([1., 0., 0.]);
        fill(&mut left, &green);
        fill(&mut right, &red);
        film.merge_film_tile(left);
        film.merge_film_tile(right);
        film.write_image(1.);
        assert_eq!(film.get_pixel_xyz([4, 4].into()), green.to_xyz());
        assert_eq!(film.get_pixel_xyz([196, 4].into()), red.to_xyz());
    }

    #[test]
    fn merge_film_tile_rainbow() {
        const WIDTH: isize = 200;
        const HEIGHT: isize = 100;
        let filter = BoxFilter::new([8., 8.].into());
        let film = Film::new(
            [WIDTH, HEIGHT].into(),
            [[0., 0.], [1., 1.]].into(),
            Box::new(filter),
            35.0,
            "target/doc/pbrt/merge_film_tile_rainbow.png".to_string(),
            1.,
            1.,
        );
        fn fill(t: &mut FilmTile) {
            for pt in t.get_pixel_bounds().iter() {
                let c = Spectrum::from_rgb([
                    pt.x as Float / WIDTH as Float,
                    pt.y as Float / HEIGHT as Float,
                    (WIDTH - pt.x) as Float / WIDTH as Float,
                ]);
                let px = t.get_pixel_mut(pt);
                px.contrib_sum = c.clone();
                px.filter_weight_sum = 1.;
            }
        }

        let mut left = film.get_film_tile(Bounds2i::from([[0, 0], [WIDTH / 2, HEIGHT]]));
        let mut right = film.get_film_tile(Bounds2i::from([[WIDTH / 2, 0], [WIDTH, HEIGHT]]));
        fill(&mut left);
        fill(&mut right);
        film.merge_film_tile(left);
        film.merge_film_tile(right);
        film.write_image(1.);
    }
}
