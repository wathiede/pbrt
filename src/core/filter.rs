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
//! Defines the interface for filter functions.

use crate::{
    core::geometry::{Point2f, Vector2f},
    Float,
};

/// Trait `Filter` describes a sampling strategy.
pub trait Filter {
    /// evaluate the filter at the given point `p`.
    fn evaluate(&self, p: Point2f) -> Float;
    /// return the radius this filter was created with.
    fn radius(&self) -> Vector2f;
    /// return the inverse of the radius this filter was created with.
    fn inv_radius(&self) -> Vector2f;
}
