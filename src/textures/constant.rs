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

//! Implements a [Texture] that always returns the given value.
//!
//! [Texture]: crate::core::texture::Texture
use std::fmt::{Debug, Formatter, Result};

use crate::{
    core::{
        interaction::SurfaceInteraction, paramset::TextureParams, spectrum::Spectrum,
        texture::Texture, transform::Transform,
    },
    Float,
};

#[derive(Clone)]
/// Implements trait [Texture] to return the given `value`.
///
/// [Texture]: crate::core::texture::Texture
pub struct ConstantTexture<T>
where
    T: Debug,
{
    value: T,
}

/// Creates new `ConstantTexture` from the given `TextureParams` with `Float` as the data type.
///
/// # Examples
/// ```
/// use pbrt::{
///     core::{
///         paramset::{testutils::make_float_param_set, TextureParams},
///         texture::Texture,
///         transform::Transform,
///     },
///     textures::constant::create_constant_float_texture,
/// };
///
/// let tp = TextureParams::new(
///     make_float_param_set("value", vec![10.]),
///     Default::default(),
///     Default::default(),
///     Default::default(),
/// );
/// let t = create_constant_float_texture(&Transform::identity(), &tp);
/// assert_eq!(10., t.evaluate(&Default::default()));
/// ```
pub fn create_constant_float_texture(
    _tex2world: &Transform,
    tp: &TextureParams,
) -> ConstantTexture<Float> {
    ConstantTexture {
        value: tp.find_float("value", 1.),
    }
}

/// Creates new `ConstantTexture` from the given `TextureParams` with `Spectrum` as the data type.
///
/// # Examples
/// ```
/// use pbrt::{
///     core::{
///         paramset::{testutils::make_spectrum_param_set, TextureParams},
///         spectrum::Spectrum,
///         texture::Texture,
///         transform::Transform,
///     },
///     textures::constant::create_constant_spectrum_texture,
/// };
///
/// let tp = TextureParams::new(
///     make_spectrum_param_set("value", vec![Spectrum::from_rgb([1., 0., 0.])]),
///     Default::default(),
///     Default::default(),
///     Default::default(),
/// );
/// let t = create_constant_spectrum_texture(&Transform::identity(), &tp);
/// assert_eq!(
///     Spectrum::from_rgb([1., 0., 0.]),
///     t.evaluate(&Default::default())
/// );
/// ```
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
    /// Create a new `ConstantTexture` with the given constant `value`.
    ///
    /// # Examples
    /// ```
    /// use pbrt::{
    ///     core::{spectrum::Spectrum, texture::Texture},
    ///     textures::constant::ConstantTexture,
    /// };
    ///
    /// let t = ConstantTexture::new(10.);
    /// assert_eq!(10., t.evaluate(&Default::default()));
    ///
    /// let t = ConstantTexture::new(Spectrum::from_rgb([1., 0., 0.]));
    /// assert_eq!(
    ///     Spectrum::from_rgb([1., 0., 0.]),
    ///     t.evaluate(&Default::default())
    /// );
    /// ```
    pub fn new(value: T) -> ConstantTexture<T> {
        ConstantTexture { value }
    }
}

impl<T> Texture<T> for ConstantTexture<T>
where
    T: Clone + Debug,
{
    /// Implements [evaluate] that just returns the same value for any `SurfaceInteraction`
    ///
    /// [evaluate]: crate::core::texture::Texture
    fn evaluate(&self, _si: &SurfaceInteraction) -> T {
        self.value.clone()
    }
}

impl<T> Debug for ConstantTexture<T>
where
    T: Debug,
{
    /// Implements [fmt] that surfaces the relevant details for the `ConstantTexture`.
    ///
    /// [fmt]: std::fmt::Debug::fmt
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ConstantTexture{{{:?}}}", &self.value)
    }
}
