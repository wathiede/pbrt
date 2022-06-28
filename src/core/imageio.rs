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

//! Utilities for writing out `Float` based image data to common image file formats.
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

use image::{self, save_buffer_with_format, ColorType, ImageError, ImageFormat};
use log::error;
use thiserror::Error;

use crate::clamp;
use crate::core::geometry::{Bounds2i, Point2i};
use crate::core::spectrum::RGBSpectrum;
use crate::gamma_correct;
use crate::Float;

/// Error type for reading images from disk.
#[derive(Debug, Error)]
pub enum Error {
    /// Error from the `image` crate.
    #[error("decoding image")]
    ImageError(#[from] ImageError),
    /// Attempt to read file type not yet implemented, but planned.
    #[error("reading '{0}' files is not yet implemented")]
    ReadNotImplemented(String),
    /// Attempt to write file type not yet implemented, but planned.
    #[error("writing '{0}' files is not yet implemented")]
    WriteNotImplemented(String),
    /// Unknown file type read/written that is not supported and isn't planned.
    #[error("unknown extension '{0}'")]
    UnknownExtension(String),
    /// Standard `io::Error` generated.
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    /// Standard `std::string::FromUtf8Error`.
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    /// Standard `std::num::ParseFloatError`.
    #[error("float error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    /// Standard `std::num::ParseIntError`.
    #[error("int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

fn to_byte(v: Float) -> u8 {
    clamp(255. * gamma_correct(v) + 0.5, 0., 255.) as u8
}

fn is_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\n' | b'\t')
}

fn read_word(buf: &mut dyn Read) -> Result<String, Error> {
    let mut byte = [0; 1];
    let mut acc = Vec::new();
    loop {
        buf.read_exact(&mut byte)?;
        if is_whitespace(byte[0]) {
            let s = String::from_utf8(acc)?;
            return Ok(s);
        }
        acc.push(byte[0]);
    }
}

fn read_image_pfm(name: &str) -> Result<(Vec<RGBSpectrum>, Point2i), Error> {
    let f = File::open(name)?;
    let mut buf = BufReader::new(f);

    let hdr = read_word(&mut buf)?;
    let n_channels = match hdr.as_str() {
        "PF" => 3,
        "Pf" => 1,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid header '{:?}'", hdr),
            )
            .into())
        }
    };
    let width: usize = read_word(&mut buf)?.parse()?;
    let height: usize = read_word(&mut buf)?.parse()?;
    let scale: f32 = read_word(&mut buf)?.parse()?;
    let n_floats = n_channels * width * height;
    let mut data = vec![0.; n_floats];
    let le = scale < 0.;
    let abs_scale = scale.abs();
    for y in (0..height).rev() {
        for x in 0..width {
            for c in 0..n_channels {
                let mut f_buf = [0; 4];
                buf.read_exact(&mut f_buf)?;
                let f = if le {
                    // Little endian file
                    f32::from_le_bytes(f_buf) * abs_scale
                } else {
                    // Big endian file
                    f32::from_be_bytes(f_buf) * abs_scale
                };
                data[c + (x + y * width) * n_channels] = f;
            }
        }
    }
    let rgb_spectrum: Vec<_> = match n_channels {
        1 => data
            .into_iter()
            .map(|f| RGBSpectrum::new(f as Float))
            .collect(),
        3 => data
            .chunks(3)
            .map(|rgb| RGBSpectrum::from_rgb([rgb[0] as Float, rgb[1] as Float, rgb[2] as Float]))
            .collect(),
        _ => unreachable!("only 1 and 3 channel images supported"),
    };
    Ok((rgb_spectrum, [width as isize, height as isize].into()))
}

/// Read and decode image at path `name`.  An error is returned on IO errors, decode errors, or
/// unsupported file types.
pub fn read_image(name: &str) -> Result<(Vec<RGBSpectrum>, Point2i), Error> {
    match Path::new(name)
        .extension()
        .expect("file has no extension")
        .to_str()
        .expect("filename not ascii")
        .to_ascii_lowercase()
        .as_str()
    {
        "png" => {
            let img = image::open(name)?;
            let rgb_img = img.to_rgb();
            let pixels: Vec<_> = rgb_img
                .pixels()
                .map(|p| {
                    let p = p.0;
                    let s = [
                        p[0] as Float / 255.,
                        p[1] as Float / 255.,
                        p[2] as Float / 255.,
                    ];
                    RGBSpectrum::from_rgb(s)
                })
                .collect();
            let dim = rgb_img.dimensions();
            Ok((pixels, Point2i::from([dim.0 as isize, dim.1 as isize])))
        }
        "exr" => Err(Error::ReadNotImplemented(".exr".to_string())),
        "tga" => Err(Error::ReadNotImplemented(".tga".to_string())),
        "pfm" => read_image_pfm(name),
        ext => Err(Error::UnknownExtension(ext.to_string())),
    }
}

fn write_image_pfm(name: &str, rgb: &[Float], resolution: Point2i) -> Result<(), Error> {
    let Point2i { x, y } = resolution;
    let (width, height) = (x, y);
    let f = File::create(name)?;
    let mut buf = BufWriter::new(f);

    let host_little_endian = 0x1234_u16.to_ne_bytes() == 0x1234_u16.to_le_bytes();
    let scale = if host_little_endian { -1. } else { 1. };
    // Write header, scale determines endianness of file on read.
    write!(buf, "PF\n{} {}\n{}\n", width, height, scale)?;

    // write the data from bottom left to upper right as specified by
    // http://netpbm.sourceforge.net/doc/pfm.html
    // The raster is a sequence of pixels, packed one after another, with no
    // delimiters of any kind. They are grouped by row, with the pixels in each
    // row ordered left to right and the rows ordered bottom to top.
    for y in (0..height).rev() {
        // in case Float is 'double', copy into a staging buffer that's
        // definitely a 32-bit float...
        for x in 0..width * 3 {
            let idx = (x + y * width * 3) as usize;
            buf.write_all(&(rgb[idx] as f32).to_ne_bytes()[..])?;
        }
    }

    buf.flush()?;
    Ok(())
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
///     1., 0., 0., //
///     1., 1., 0., //
///     0., 0., 1., //
///     0., 1., 0., //
/// ];
/// let b = Bounds2i::from([[0, 0], [2, 2]]);
/// let res = Point2i::from([2, 2]);
/// write_image("target/doc/pbrt/test.png", &data, b, res);
/// ```
pub fn write_image(name: &str, rgb: &[Float], output_bounds: Bounds2i, _total_resolution: Point2i) {
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
            let rgb8: Vec<u8> = rgb.iter().map(|f| to_byte(*f)).collect();

            if let Err(err) = save_buffer_with_format(
                name,
                &rgb8,
                resolution.x as u32,
                resolution.y as u32,
                ColorType::RGB(8),
                ImageFormat::PNG,
            ) {
                error!("Failed to write PNG to '{}': {}", name, err);
            }
        }
        "exr" => unimplemented!("writing .exr files is not implemented"),
        "tga" => unimplemented!("writing .tga files is not implemented"),
        "pfm" => {
            if let Err(err) =
                write_image_pfm(name, rgb, Point2i::from([resolution.x, resolution.y]))
            {
                error!("Failed to write PFM to '{}': {}", name, err);
            }
        }
        ext => error!("unknown file extension {}", ext),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use tempfile::Builder;

    struct TestImage {
        name: String,
        res: Point2i,
        bounds: Bounds2i,
        pixels: Vec<Float>,
    }

    fn make_image(ext: &str) -> TestImage {
        let res: Point2i = [64, 64].into();
        let bounds: Bounds2i = [[0, 0].into(), res].into();
        let pixels: Vec<Float> = bounds
            .iter()
            .map(|Point2i { x, y }| {
                vec![x as Float / res.x as Float, y as Float / res.y as Float, 1.].into_iter()
            })
            .flatten()
            .collect();
        let f = Builder::new()
            .prefix("imageio-roundtrip")
            // Use pfm over png, because 8-bit formats apply gamma correction on write, therefore
            // roundtrip test fails.
            .suffix(ext)
            .tempfile()
            .expect("failed to create NamedTempFile");
        let name = f.path().to_string_lossy().to_string();
        TestImage {
            name,
            res,
            bounds,
            pixels,
        }
    }

    #[test]
    fn roundtrip_png() {
        let test_img = make_image(".png");
        write_image(
            &test_img.name,
            &test_img.pixels,
            test_img.bounds,
            test_img.res,
        );
        // To keep image around for inspection, force exit.  This causes the unit test to leak
        // images.
        // dbg!(&name);
        // std::process::exit(1);
        match read_image(&test_img.name) {
            Ok((read_spectrum, read_res)) => {
                let read_pixels: Vec<Float> = read_spectrum
                    .into_iter()
                    .map(|s| s.to_rgb().to_vec().into_iter())
                    .flatten()
                    .collect();
                // 8-bit image formats gamma correct on save, so we need to gamma correct the
                // original pixels before comparing.
                let test_pixels: Vec<_> = test_img
                    .pixels
                    .into_iter()
                    .map(|p| to_byte(p) as Float / 255.)
                    .collect();
                assert_eq!(test_img.res, read_res);
                // Sample image for easier to digest failures.
                assert_eq!(&test_pixels[..12], &read_pixels[..12]);
                // Still compare the whole image for correctness.
                assert_eq!(test_pixels, read_pixels);
            }
            Err(e) => panic!("{}", e.to_string()),
        }
    }

    #[test]
    fn roundtrip_pfm() {
        let test_img = make_image(".pfm");
        write_image(
            &test_img.name,
            &test_img.pixels,
            test_img.bounds,
            test_img.res,
        );
        // To keep image around for inspection, force exit.  This causes the unit test to leak
        // images.
        // dbg!(&name);
        // std::process::exit(1);
        match read_image(&test_img.name) {
            Ok((read_spectrum, read_res)) => {
                let read_pixels: Vec<Float> = read_spectrum
                    .into_iter()
                    .map(|s| s.to_rgb().to_vec().into_iter())
                    .flatten()
                    .collect();
                assert_eq!(test_img.res, read_res);
                // Sample image for easier to digest failures.
                assert_eq!(&test_img.pixels[..12], &read_pixels[..12]);
                // Still compare the whole image for correctness.
                assert_eq!(test_img.pixels, read_pixels);
            }
            Err(e) => panic!("{}", e.to_string()),
        }
    }
}
