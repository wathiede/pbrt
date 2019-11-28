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

use crate::core::pbrt::Float;

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

#[derive(Debug, Clone, PartialEq)]
pub struct RGBSpectrum {
    c: [Float; 3],
}

common_implementation!(RGBSpectrum, 3);

#[cfg(not(feature = "sampled-spectrum"))]
pub type Spectrum = RGBSpectrum;
#[cfg(feature = "sampled-spectrum")]
pub type Spectrum = SampledSpectrum;
