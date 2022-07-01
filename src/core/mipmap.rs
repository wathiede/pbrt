// Copyright 2020 Google LLC
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

//! Module mimmap provides tools for building image pyramids for efficient texture lookups.
use lazy_static::lazy_static;

use crate::{core::geometry::Point2i, Float};

/// ImageWrap describes the mipmap sampling behavior when the sample is outside the range of [0,
/// 1].
#[derive(Debug)]
pub enum ImageWrap {
    /// Wrap around.
    Repeat,
    /// Return black pixels.
    Black,
    /// Clamp to edge of texture.
    Clamp,
}

/// MIPMap holds an image pyramid to efficiently sample texture maps at various resolutions.
#[derive(Debug)]
pub struct MIPMap<T> {
    do_trilinear: bool,
    max_anisotropy: Float,
    wrap_mode: ImageWrap,
    resolution: Point2i,
    // TODO(wathiede): C++ uses a BlockedArray here, which is fancy.  Fake it for the time being.
    pyramid: Vec<Vec<T>>,
}

const WEIGHT_LUT_SIZE: usize = 128;
lazy_static! {
    static ref WEIGHT_LUT: Vec<Float> = (0..WEIGHT_LUT_SIZE)
        .map(|i| {
            const ALPHA: Float = 2.;
            let r2 = i as Float / (WEIGHT_LUT_SIZE - 1) as Float;
            (-ALPHA * r2).exp() - (-ALPHA).exp()
        })
        .collect::<Vec<Float>>();
}

impl<T> MIPMap<T> {
    // TODO(wathiede): add builder when we need to set do_trilinear, max_anisotropy, or wrap_mode.
    /// Create a MIPMap for the texture represented by `data` of size `resolution`.
    pub fn new(resolution: &Point2i, data: Vec<T>) -> Self {
        let _ = MIPMap {
            resolution: *resolution,
            // TODO(wathiede): build actual pyramid,
            pyramid: vec![data],
            do_trilinear: false,
            max_anisotropy: 8.,
            wrap_mode: ImageWrap::Repeat,
        };
        todo!("MIPMap::new()");
    }
}
