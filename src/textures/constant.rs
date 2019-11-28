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
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::core::interaction::SurfaceInteraction;
use crate::core::paramset::TextureParams;
use crate::core::pbrt::Float;
use crate::core::spectrum::Spectrum;
use crate::core::texture::Texture;
use crate::core::transform::Transform;

#[derive(Clone)]
pub struct ConstantTexture<T>
where
    T: Debug,
{
    value: T,
}

pub fn create_constant_float_texture(
    _tex2world: &Transform,
    tp: &TextureParams,
) -> ConstantTexture<Float> {
    ConstantTexture {
        value: tp.find_float("value", 1.),
    }
}

pub fn create_constant_spectrum_texture(
    _tex2world: &Transform,
    tp: &TextureParams,
) -> ConstantTexture<Spectrum> {
    ConstantTexture {
        value: tp.find_spectrum("value", Spectrum::from(1.)),
    }
}

impl<T> ConstantTexture<T>
where
    T: Debug,
{
    pub fn new(value: T) -> ConstantTexture<T> {
        ConstantTexture { value }
    }
}

impl<T> Texture<T> for ConstantTexture<T>
where
    T: Clone + Debug,
{
    fn evaluate(&self, _si: &SurfaceInteraction) -> T {
        self.value.clone()
    }
}

impl<T> Debug for ConstantTexture<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ConstantTexture{{{:?}}}", &self.value)
    }
}
