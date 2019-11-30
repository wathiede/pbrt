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

//! Types and utilities for dealing with 2D and 3D, integer and float data types.
use std::fmt;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use crate::float;
use crate::Float;

mod bounds;
pub use crate::core::geometry::bounds::Bounds2f;
pub use crate::core::geometry::bounds::Bounds2i;

mod normal;
pub use crate::core::geometry::normal::Normal3f;

mod point;
pub use crate::core::geometry::point::Point2f;
pub use crate::core::geometry::point::Point2i;
pub use crate::core::geometry::point::Point3f;
pub use crate::core::geometry::point::Point3i;

mod vector;
pub use crate::core::geometry::vector::Vector2f;
pub use crate::core::geometry::vector::Vector2i;
pub use crate::core::geometry::vector::Vector3f;
pub use crate::core::geometry::vector::Vector3i;

/// Trait for ensuring methods present on only `{float}` or `{integer}` types have appropriate
/// implementations as necessary for this crate.
pub trait Number
where
    Self: std::marker::Sized
        + Copy
        + fmt::Display
        + std::cmp::PartialOrd
        + Add<Output = Self>
        + Div<Output = Self>
        + Mul<Output = Self>
        + Sub<Output = Self>,
{
    /// Returns true if this value is NaN.
    fn is_nan(self) -> bool;
    /// Returns the smallest value this type can hold.
    fn min_value() -> Self;
    /// Returns the largest value this type can hold.
    fn max_value() -> Self;
}

impl Number for Float {
    fn is_nan(self) -> bool {
        self.is_nan()
    }
    fn min_value() -> Self {
        float::MIN
    }
    fn max_value() -> Self {
        float::MAX
    }
}

impl Number for isize {
    fn is_nan(self) -> bool {
        false
    }
    fn min_value() -> Self {
        std::isize::MIN
    }
    fn max_value() -> Self {
        std::isize::MAX
    }
}
