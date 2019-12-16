use std::path::Path;

use image::save_buffer_with_format;
use image::ColorType;
use image::ImageFormat;
use log::error;

use crate::clamp;
use crate::core::geometry::Bounds2i;
use crate::core::geometry::Point2i;
use crate::gamma_correct;
use crate::Float;

fn to_byte(v: Float) -> u8 {
    clamp(255. * gamma_correct(v) + 0.5, 0., 255.) as u8
}

/// Writes the RGB pixel data in `rgb` to `name`. File format is chosen based on the files
/// extension, only PNG is currently supported.
///
/// # Examples
/// ```
/// use pbrt::core::geometry::Bounds2i;
/// use pbrt::core::geometry::Point2i;
/// use pbrt::core::imageio::write_image;
///
/// let data = vec![
///     255., 0., 0., //
///     255., 255., 0., //
///     0., 0., 255., //
///     0., 255., 0., //
/// ];
/// let b = Bounds2i::from([[0, 0], [2, 2]]);
/// let res = Point2i::from([2, 2]);
/// write_image("target/doc/pbrt/test.png", &data, b, res);
/// ```
pub fn write_image(
    name: &str,
    rgb: &Vec<Float>,
    output_bounds: Bounds2i,
    total_resolution: Point2i,
) {
    let resolution = output_bounds.diagonal();
    match Path::new(name)
        .extension()
        .expect("file has no extension")
        .to_str()
        .expect("filename not ascii")
        .to_ascii_lowercase()
        .as_str()
    {
        "png" => {
            let mut rgb8: Vec<u8> = rgb.iter().map(|f| to_byte(*f)).collect();

            if let Err(err) = save_buffer_with_format(
                name,
                &rgb8,
                resolution.x as u32,
                resolution.y as u32,
                ColorType::RGB(8),
                ImageFormat::PNG,
            ) {
                error!("Failed to write PNG To {}: {}", name, err);
            }
        }
        "exr" => unimplemented!("writing .exr files is not implemented"),
        "tga" => unimplemented!("writing .tga files is not implemented"),
        "pfm" => unimplemented!("writing .pfm files is not implemented"),
        ext @ _ => error!("unknown file extension {}", ext),
    }
}