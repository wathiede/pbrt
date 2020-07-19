// Copyright 2018 Google LLC
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

//! The spectrum module houses two main types, [RGBSpectrum] and [SampledSpectrum] used to represent color. The default [Spectrum] type is defined as `RGBSpectrum` or `SampledSpectrum` based on the compile time feature `sampled-spectrum`.
//!
//! [RGBSpectrum]: crate::core::spectrum::RGBSpectrum
//! [SampledSpectrum]: crate::core::spectrum::SampledSpectrum
//! [Spectrum]: crate::core::spectrum::Spectrum
use std::cmp;
use std::fmt;
use std::ops::{Mul, MulAssign};

use crate::Float;

/// Spectrum type, used when converting between RGB and [SampledSpectrum]
#[derive(Debug)]
pub enum SpectrumType {
    /// Use reflectance coefficients
    Reflectance,
    /// Use illuminant coefficients
    Illuminant,
}

macro_rules! common_implementation {
    ($($t:ty, $n:expr)*) => ($(
impl Default for $t {
    fn default() -> $t {
     Self {
            c: [0.; $n],
        }
    }
}

impl From<Float> for $t {
    fn from(value: Float) -> Self {
        Self {
            c: [value; $n],
        }
    }
}

impl MulAssign for $t {
    fn mul_assign(&mut self, rhs:Self) {
        self.c.iter_mut().zip(rhs.c.iter()).enumerate().for_each(|(i, (l, r))| *l *= r);
    }
}

impl Mul for $t {
    type Output = Self;
    fn mul(self, rhs:Self) -> Self::Output {
        let mut tmp = [0.; $n];
        self.c.iter().zip(rhs.c.iter()).enumerate().for_each(|(i, (l,r))| tmp[i]=l*r);
        Self{
            c:tmp,
        }
    }
}

impl $t {
    #[allow(dead_code)]
    fn has_nans(&self) -> bool {
        for i in 0..$n {
            if self.c[i].is_nan() {
                return true;
            }
        }
        false
    }
}

    )*)
}

const N_SPECTRAL_SAMPLES: usize = 60;
/// `SampledSpectrum` is a spectrum represented by `N_SPECTRAL_SAMPLES` (currently 60) values
/// evenly spread across 400 nm to 700 nm.
pub struct SampledSpectrum {
    c: [Float; N_SPECTRAL_SAMPLES],
}

common_implementation!(SampledSpectrum, N_SPECTRAL_SAMPLES);

impl fmt::Debug for SampledSpectrum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SampledSpectrum({:?})", &self.c[..])
    }
}

impl cmp::PartialEq for SampledSpectrum {
    fn eq(&self, other: &SampledSpectrum) -> bool {
        let mut it = self.c.iter().zip(other.c.iter());
        while let Some((&l, &r)) = it.next() {
            if l != r {
                return false;
            }
        }
        true
    }
}

/// Convert tristimulus values in the XYZ color space (as defined by CIE) matching the human eye's
/// response to RGB values in the sRGB color space.
pub fn xyz_to_rgb(xyz: [Float; 3]) -> [Float; 3] {
    [
        3.240479 * xyz[0] - 1.537150 * xyz[1] - 0.498535 * xyz[2],
        -0.969256 * xyz[0] + 1.875991 * xyz[1] + 0.041556 * xyz[2],
        0.055648 * xyz[0] - 0.204043 * xyz[1] + 1.057311 * xyz[2],
    ]
}

/// Convert tristimulus values in the sRGB color space values to the XYZ color space (as defined by
/// CIE) matching the human eye's response.
pub fn rgb_to_xyz(rgb: [Float; 3]) -> [Float; 3] {
    [
        0.412453 * rgb[0] + 0.357580 * rgb[1] + 0.180423 * rgb[2],
        0.212671 * rgb[0] + 0.715160 * rgb[1] + 0.072169 * rgb[2],
        0.019334 * rgb[0] + 0.119193 * rgb[1] + 0.950227 * rgb[2],
    ]
}

/// `RGBSpectrum` is a sample implemented with 3 values at red, green and blue points in the
/// spectrum.  Values stored are in the range [0., 1.].
#[derive(Debug, Clone, PartialEq)]
pub struct RGBSpectrum {
    c: [Float; 3],
}

common_implementation!(RGBSpectrum, 3);

#[cfg(not(feature = "sampled-spectrum"))]
/// Define the `Spectrum` type to be `RGBSpectrum` when compiling without the `sampled-spectrum` feature enabled.
pub type Spectrum = RGBSpectrum;

impl RGBSpectrum {
    /// Create an RGBSpectrum with each component set to `v`.
    pub fn new(v: Float) -> RGBSpectrum {
        RGBSpectrum { c: [v, v, v] }
    }
    /// Create an RGBSpectrum from Self.  This is a no-op on RGBSpectrum, but exists for a unified
    /// API with SampledSpectrum.
    pub fn to_rgb_spectrum(&self) -> RGBSpectrum {
        RGBSpectrum { c: self.c }
    }
    /// extract this `RGBSpectrum`'s value in the XYZ color space.
    pub fn to_xyz(&self) -> [Float; 3] {
        rgb_to_xyz(self.c)
    }

    /// extract this `RGBSpectrum`'s value in the RGB color space.
    pub fn to_rgb(&self) -> [Float; 3] {
        self.c
    }

    /// create an `RGBSpectrum` from the given tristimulus values in sRGB color space.
    pub fn from_rgb(c: [Float; 3]) -> RGBSpectrum {
        let s = RGBSpectrum { c };
        debug_assert!(!s.has_nans(), "c {:?}", s);
        s
    }

    /// create an `RGBSpectrum` from the given tristimulus values in XYZ color space.
    pub fn from_xyz(c: [Float; 3]) -> RGBSpectrum {
        let rgb = xyz_to_rgb(c);
        let s = RGBSpectrum { c: rgb };
        debug_assert!(!s.has_nans(), "c {:?}", s);
        s
    }
}

#[cfg(feature = "sampled-spectrum")]
/// Define the `Spectrum` type to be `SampledSpectrum` when compiling with the `sampled-spectrum` feature enabled.
pub type Spectrum = SampledSpectrum;
