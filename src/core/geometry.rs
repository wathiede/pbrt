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
pub use crate::core::geometry::bounds::Bounds2;
pub use crate::core::geometry::bounds::Bounds2f;
pub use crate::core::geometry::bounds::Bounds2i;
pub use crate::core::geometry::bounds::Bounds3;
pub use crate::core::geometry::bounds::Bounds3f;
pub use crate::core::geometry::bounds::Bounds3i;

mod normal;
pub use crate::core::geometry::normal::Normal3;
pub use crate::core::geometry::normal::Normal3f;

mod point;
pub use crate::core::geometry::point::Point2;
pub use crate::core::geometry::point::Point2f;
pub use crate::core::geometry::point::Point2i;
pub use crate::core::geometry::point::Point3;
pub use crate::core::geometry::point::Point3f;
pub use crate::core::geometry::point::Point3i;

mod vector;
pub use crate::core::geometry::vector::cross;
pub use crate::core::geometry::vector::Vector2;
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
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Number;
    /// use pbrt::float::NAN;
    /// use pbrt::Float;
    ///
    /// let i: isize = 1;
    /// let f1: Float = 1.;
    /// let f2 = NAN;
    /// assert_eq!(Number::is_nan(f1), false);
    /// assert_eq!(Number::is_nan(f2), true);
    /// assert_eq!(Number::is_nan(i), false);
    /// ```
    fn is_nan(self) -> bool;

    /// Returns the smallest value this type can hold.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Number;
    /// use pbrt::Float;
    ///
    /// #[cfg(not(feature = "float-as-double"))]
    /// assert_eq!(<Float as Number>::min_value(), -3.4028235e+38);
    /// #[cfg(feature = "float-as-double")]
    /// assert_eq!(<Float as Number>::min_value(), -1.7976931348623157e+308);
    /// assert_eq!(<isize as Number>::min_value(), -9223372036854775808);
    /// ```
    fn min_value() -> Self;

    /// Returns the largest value this type can hold.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Number;
    /// use pbrt::Float;
    ///
    /// #[cfg(not(feature = "float-as-double"))]
    /// assert_eq!(<Float as Number>::max_value(), 3.4028235e+38);
    /// #[cfg(feature = "float-as-double")]
    /// assert_eq!(<Float as Number>::max_value(), 1.7976931348623157e+308);
    /// assert_eq!(<isize as Number>::max_value(), 9223372036854775807);
    /// ```
    fn max_value() -> Self;

    /// Returns the maximum of self or other.  No special care is taken for NaN and infinity.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Number;
    /// use pbrt::Float;
    ///
    /// let x: Float = 1.;
    /// let y: Float = 2.;
    ///
    /// assert_eq!(Number::max(x, y), y);
    /// assert_eq!(Number::max(x, y), y);
    ///
    /// let a: isize = 1;
    /// let b: isize = 2;
    ///
    /// assert_eq!(Number::max(a, b), b);
    /// assert_eq!(Number::max(a, b), b);
    ///
    /// fn bigger<T>(m: T, n: T) -> T
    /// where
    ///     T: Number,
    /// {
    ///     m.max(n)
    /// }
    /// assert_eq!(bigger(a, b), b)
    /// ```
    fn max(self, other: Self) -> Self;

    /// Returns the minimum of self or other.  No special care is taken for NaN and infinity.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Number;
    /// use pbrt::Float;
    ///
    /// let x: Float = 1.;
    /// let y: Float = 2.;
    ///
    /// assert_eq!(Number::min(x, y), x);
    /// assert_eq!(Number::min(x, y), x);
    ///
    /// let a: isize = 1;
    /// let b: isize = 2;
    ///
    /// assert_eq!(Number::min(a, b), a);
    /// assert_eq!(Number::min(a, b), a);
    ///
    /// fn smaller<T>(m: T, n: T) -> T
    /// where
    ///     T: Number,
    /// {
    ///     m.min(n)
    /// }
    /// assert_eq!(smaller(a, b), a)
    /// ```
    fn min(self, other: Self) -> Self;
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
    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }
    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
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
    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }
    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }
}
