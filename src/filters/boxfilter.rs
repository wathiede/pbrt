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
//! Defines a box filter that implements [Filter].
//! Note: This is named `BoxFilter` and not `Box` due to `Box` being a reserved word in rust.
//!
//! [Filter]: crate::core::filter::Filter
use crate::core::filter::Filter;
use crate::core::geometry::Point2f;
use crate::core::geometry::Vector2f;
use crate::Float;

/// Filter that returns 1. within the configured `radius`.
pub struct BoxFilter {
    radius: Vector2f,
    inv_radius: Vector2f,
}

impl BoxFilter {
    /// Create a new `BoxFilter` with the given `radius`.
    pub fn new(radius: Vector2f) -> Self {
        Self {
            radius,
            inv_radius: [1. / radius.x, 1. / radius.y].into(),
        }
    }
}

impl Filter for BoxFilter {
    /// evalute the filter at the given point `p`.
    fn evaluate(&self, p: Point2f) -> Float {
        1.
    }
    /// return the radius this filter was created with.
    fn radius(&self) -> Vector2f {
        self.radius
    }
    /// return the inv_radius this filter was created with.
    fn inv_radius(&self) -> Vector2f {
        self.inv_radius
    }
}
