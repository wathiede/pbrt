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

//! Traits and helper types to define lighting.

use std::fmt::Debug;

use crate::core::medium::MediumInterface;

/// Flags for the various light types.
#[derive(Debug)]
pub enum LightFlags {
    /// Light uses delta sampling, type position.
    DeltaPosition,
    /// Light uses delta sampling, type direction.
    DeltaDirection,
    /// Is an area light.
    Area,
    /// Is a light at infinite distance.
    Infinite,
}

/// Stub type for flushing out [PbrtAPI].  TODO(wathiede): actually implement and document.
///
/// [PbrtAPI]: crate::core::api::PbrtAPI
pub trait Light: Debug {}

/// LightData holds data common to various `Light` implementations.
#[derive(Debug)]
pub struct LightData {
    flags: LightFlags,
    n_samples: isize,
    medium_interface: MediumInterface,
}

// TODO(wathiede): figure out how to do:
// STAT_COUNTER("Scene/Lights", numLights);
// STAT_COUNTER("Scene/AreaLights", numAreaLights);
impl LightData {
    /// Construct `LightData` from given parameters.
    pub fn new(
        flags: LightFlags,
        n_samples: isize,
        medium_interface: MediumInterface,
    ) -> LightData {
        // TODO(wathiede): increment STAT_COUNTER for numLights.
        LightData {
            flags,
            n_samples,
            medium_interface,
        }
    }
}
