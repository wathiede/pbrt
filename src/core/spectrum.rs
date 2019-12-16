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
use std::cmp;
use std::fmt;

use crate::Float;

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

impl $t {
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

pub fn xyz_to_rgb(xyz: [Float; 3]) -> [Float; 3] {
    [
        3.240479 * xyz[0] - 1.537150 * xyz[1] - 0.498535 * xyz[2],
        -0.969256 * xyz[0] + 1.875991 * xyz[1] + 0.041556 * xyz[2],
        0.055648 * xyz[0] - 0.204043 * xyz[1] + 1.057311 * xyz[2],
    ]
}

pub fn rgb_to_xyz(rgb: [Float; 3]) -> [Float; 3] {
    [
        0.412453 * rgb[0] + 0.357580 * rgb[1] + 0.180423 * rgb[2],
        0.212671 * rgb[0] + 0.715160 * rgb[1] + 0.072169 * rgb[2],
        0.019334 * rgb[0] + 0.119193 * rgb[1] + 0.950227 * rgb[2],
    ]
}

#[derive(Debug, Clone, PartialEq)]
pub struct RGBSpectrum {
    c: [Float; 3],
}

common_implementation!(RGBSpectrum, 3);

#[cfg(not(feature = "sampled-spectrum"))]
pub type Spectrum = RGBSpectrum;

impl RGBSpectrum {
    pub fn to_xyz(&self) -> [Float; 3] {
        rgb_to_xyz(self.c)
    }
    pub fn from_rgb(c: [Float; 3]) -> RGBSpectrum {
        let s = RGBSpectrum { c };
        debug_assert!(!s.has_nans(), "c {:?}", s);
        s
    }
}

#[cfg(feature = "sampled-spectrum")]
pub type Spectrum = SampledSpectrum;
