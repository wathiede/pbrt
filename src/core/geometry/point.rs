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
use std::ops::Sub;

use crate::core::geometry::Number;
use crate::Float;

/// Generic type for any 2D point.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Point2<T>
where
    T: Number,
{
    /// The x coordinate of this point.
    pub x: T,
    /// The y coordinate of this point.
    pub y: T,
}

impl<T> From<[T; 2]> for Point2<T>
where
    T: Number,
{
    fn from(xy: [T; 2]) -> Self {
        Point2 { x: xy[0], y: xy[1] }
    }
}

impl<T> From<(T, T)> for Point2<T>
where
    T: Number,
{
    fn from((x, y): (T, T)) -> Self {
        Point2 { x, y }
    }
}

impl<T> fmt::Display for Point2<T>
where
    T: Number,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {}, {} ]", self.x, self.y)
    }
}

impl<T> Sub for Point2<T>
where
    T: Number,
{
    type Output = Self;

    /// Implement `-` for Point2<T>
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Point2i;
    ///
    /// let p1: Point2i = [2, 3].into();
    /// let p2: Point2i = [4, 5].into();
    /// assert_eq!(p2 - p1, [2, 2].into());
    ///
    /// use pbrt::core::geometry::Point2f;
    ///
    /// let p1: Point2f = [2., 3.].into();
    /// let p2: Point2f = [4., 5.].into();
    /// assert_eq!(p2 - p1, [2., 2.].into());
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Point2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// 2D point type with `Float` members.
pub type Point2f = Point2<Float>;
/// 2D point type with `isize` members.
pub type Point2i = Point2<isize>;

/// Generic type for any 3D point.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Point3<T> {
    /// The x coordinate of this point.
    pub x: T,
    /// The y coordinate of this point.
    pub y: T,
    /// The z coordinate of this point.
    pub z: T,
}

/// 2D point type with `Float` members.
pub type Point3f = Point3<Float>;
/// 3D point type with `isize` members.
pub type Point3i = Point3<isize>;
