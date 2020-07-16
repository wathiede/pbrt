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

//! [Light] implementation for infinite area light.
//!
//! [Light]: crate::core::light::Light
use std::sync::Arc;

use crate::core::light::Light;
use crate::core::paramset::ParamSet;
use crate::core::transform::Transform;

#[derive(Debug)]
/// InfiniteAreaLight represents a light infinitely far away that surrounds the entire scene.
pub struct InfiniteAreaLight {
    /*
// InfiniteAreaLight Private Data
std::unique_ptr<MIPMap<RGBSpectrum>> Lmap;
Point3f worldCenter;
Float worldRadius;
std::unique_ptr<Distribution2D> distribution;
 */}

impl Light for InfiniteAreaLight {}

/// Creates an InfiniteAreaLight with the given `Transform` and parameters.
pub fn create_infinite_light(
    _light2world: &Transform,
    _params: &ParamSet,
) -> Arc<InfiniteAreaLight> {
    todo!("lights::infinite::create_infinite_light");
}
