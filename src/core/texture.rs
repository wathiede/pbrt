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

//! Defines the trait all texture algorithms must implement.  See [textures] for the currently
//! implemented algorithms.
//!
//! [textures]: crate::textures
use std::fmt::Debug;

use crate::core::interaction::SurfaceInteraction;

/// The `Texture` trait allows for sampling a material that varies across the surface of an object.
pub trait Texture<T>: Debug
where
    T: Debug,
{
    /// `evaluate` the texture function at given surface location.
    fn evaluate(&self, _si: &SurfaceInteraction) -> T;
}

/// Helper definition so boxed `Texture`s are usable as `Texture` trait objects.
impl<T> Texture<T> for Box<dyn Texture<T>>
where
    T: Debug,
{
    fn evaluate(&self, si: &SurfaceInteraction) -> T {
        (**self).evaluate(si)
    }
}
