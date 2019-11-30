use log::info;

use crate::core::filter::Filter;
use crate::core::geometry::Bounds2f;
use crate::core::geometry::Bounds2i;
use crate::core::geometry::Point2f;
use crate::core::geometry::Point2i;
use crate::Float;

const FILTER_TABLE_WIDTH: usize = 16;

struct Pixel {
    // TODO(wathiede): implement
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
}
