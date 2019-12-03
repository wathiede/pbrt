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

#![deny(missing_docs)]
//! Types and utilities for dealing with 2D and 3D, integer and float data types.
use std::fmt;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use crate::core::geometry::vector::Vector2;
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

impl<T> Point2<T>
where
    T: Number,
{
    /// Create a new `Point2` with the min `x` and `y` values from p1 & p2.
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Point2i;
    ///
    /// let p1 = Point2i::from([2, 8]);
    /// let p2 = Point2i::from([7, 3]);
    /// assert_eq!(Point2i::min(p1, p2), Point2i::from([2, 3]));
    /// ```
    pub fn min(p1: Point2<T>, p2: Point2<T>) -> Point2<T> {
        let x = p1.x.min(p2.x);
        let y = p1.y.min(p2.y);
        Point2 { x, y }
    }

    /// Create a new `Point2` with the max `x` and `y` values from p1 & p2.
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Point2i;
    ///
    /// let p1 = Point2i::from([2, 8]);
    /// let p2 = Point2i::from([7, 3]);
    /// assert_eq!(Point2i::max(p1, p2), Point2i::from([7, 8]));
    /// ```
    pub fn max(p1: Point2<T>, p2: Point2<T>) -> Point2<T> {
        let x = p1.x.max(p2.x);
        let y = p1.y.max(p2.y);
        Point2 { x, y }
    }
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

impl<T> Div<T> for Point2<T>
where
    T: Number,
{
    type Output = Self;

    /// Implement `/` for Point2<T> / T
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Point2i;
    ///
    /// let p: Point2i = [8, 16].into();
    /// assert_eq!(p / 2, [4, 8].into());
    ///
    /// use pbrt::core::geometry::Point2f;
    ///
    /// let p: Point2f = [8., 16.].into();
    /// assert_eq!(p / 2., [4., 8.].into());
    /// ```
    fn div(self, rhs: T) -> Self::Output {
        Point2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> Mul<T> for Point2<T>
where
    T: Number,
{
    type Output = Self;

    /// Implement `*` for Point2<T> * T
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Point2i;
    ///
    /// let p: Point2i = [8, 16].into();
    /// assert_eq!(p * 2, [16, 32].into());
    ///
    /// use pbrt::core::geometry::Point2f;
    ///
    /// let p: Point2f = [8., 16.].into();
    /// assert_eq!(p * 2., [16., 32.].into());
    /// ```
    fn mul(self, rhs: T) -> Self::Output {
        Point2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
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

impl<T> Sub<Vector2<T>> for Point2<T>
where
    T: Number,
{
    type Output = Self;

    /// Implement `-` for Point2<T> - Vector2<T>
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Point2i;
    /// use pbrt::core::geometry::Vector2i;
    ///
    /// let p1: Point2i = [4, 5].into();
    /// let v1: Vector2i = [2, 3].into();
    /// assert_eq!(p1 - v1, Point2i::from([2, 2]));
    ///
    /// use pbrt::core::geometry::Point2f;
    /// use pbrt::core::geometry::Vector2f;
    ///
    /// let p1: Point2f = [4., 5.].into();
    /// let v1: Vector2f = [2., 3.].into();
    /// assert_eq!(p1 - v1, Point2f::from([2., 2.]));
    /// ```
    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        Point2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Add for Point2<T>
where
    T: Number,
{
    type Output = Self;

    /// Implement `+` for Point2<T>
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Point2i;
    ///
    /// let p1: Point2i = [2, 3].into();
    /// let p2: Point2i = [4, 5].into();
    /// assert_eq!(p2 + p1, [6, 8].into());
    ///
    /// use pbrt::core::geometry::Point2f;
    ///
    /// let p1: Point2f = [2., 3.].into();
    /// let p2: Point2f = [4., 5.].into();
    /// assert_eq!(p2 + p1, [6., 8.].into());
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Point2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Add<Vector2<T>> for Point2<T>
where
    T: Number,
{
    type Output = Self;

    /// Implement `+` for Point2<T> + Vector2<T>
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Point2i;
    /// use pbrt::core::geometry::Vector2i;
    ///
    /// let p1: Point2i = [4, 5].into();
    /// let v1: Vector2i = [2, 3].into();
    /// assert_eq!(p1 + v1, Point2i::from([6, 8]));
    ///
    /// use pbrt::core::geometry::Point2f;
    /// use pbrt::core::geometry::Vector2f;
    ///
    /// let p1: Point2f = [4., 5.].into();
    /// let v1: Vector2f = [2., 3.].into();
    /// assert_eq!(p1 + v1, Point2f::from([6., 8.]));
    /// ```
    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Point2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// 2D point type with `Float` members.
pub type Point2f = Point2<Float>;

impl Point2f {
    /// Returns the floor of each value as a new point.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Point2f;
    ///
    /// let p: Point2f = [1.5, 2.5].into();
    /// assert_eq!(p.floor(), [1., 2.].into());
    /// ```
    pub fn floor(&self) -> Point2f {
        [self.x.floor(), self.y.floor()].into()
    }

    /// Returns the ceiling of each value as a new point.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Point2f;
    ///
    /// let p: Point2f = [1.5, 2.5].into();
    /// assert_eq!(p.ceil(), [2., 3.].into());
    /// ```
    pub fn ceil(&self) -> Point2f {
        [self.x.ceil(), self.y.ceil()].into()
    }
}

impl From<Point2i> for Point2f {
    fn from(p: Point2i) -> Self {
        Self {
            x: p.x as Float,
            y: p.y as Float,
        }
    }
}

/// 2D point type with `isize` members.
pub type Point2i = Point2<isize>;

impl From<Point2f> for Point2i {
    fn from(p: Point2f) -> Self {
        Self {
            x: p.x as isize,
            y: p.y as isize,
        }
    }
}

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

impl<T> Point3<T>
where
    T: Number,
{
    /// Create a new `Point3` with the min `x`, `y` and `z` values from p1 & p2.
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Point3i;
    ///
    /// let p1 = Point3i::from([2, 4, 8]);
    /// let p2 = Point3i::from([7, 5, 3]);
    /// assert_eq!(Point3i::min(p1, p2), Point3i::from([2, 4, 3]));
    /// ```
    pub fn min(p1: Point3<T>, p2: Point3<T>) -> Point3<T> {
        let x = p1.x.min(p2.x);
        let y = p1.y.min(p2.y);
        let z = p1.z.min(p2.z);
        Point3 { x, y, z }
    }

    /// Create a new `Point2` with the max `x`, `y` and `z` values from p1 & p2.
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Point3i;
    ///
    /// let p1 = Point3i::from([2, 4, 8]);
    /// let p2 = Point3i::from([7, 5, 3]);
    /// assert_eq!(Point3i::max(p1, p2), Point3i::from([7, 5, 8]));
    /// ```
    pub fn max(p1: Point3<T>, p2: Point3<T>) -> Point3<T> {
        let x = p1.x.max(p2.x);
        let y = p1.y.max(p2.y);
        let z = p1.z.max(p2.z);
        Point3 { x, y, z }
    }
}
impl<T> From<[T; 3]> for Point3<T>
where
    T: Number,
{
    fn from(xyz: [T; 3]) -> Self {
        Point3 {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        }
    }
}

/// 2D point type with `Float` members.
pub type Point3f = Point3<Float>;

impl Point3f {
    /// Returns the floor of each value as a new point.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Point3f;
    ///
    /// let p: Point3f = [1.5, 2.5, 3.5].into();
    /// assert_eq!(p.floor(), [1., 2., 3.].into());
    /// ```
    pub fn floor(&self) -> Point3f {
        [self.x.floor(), self.y.floor(), self.z.floor()].into()
    }

    /// Returns the ceiling of each value as a new point.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Point3f;
    ///
    /// let p: Point3f = [1.5, 2.5, 3.5].into();
    /// assert_eq!(p.ceil(), [2., 3., 4.].into());
    /// ```
    pub fn ceil(&self) -> Point3f {
        [self.x.ceil(), self.y.ceil(), self.z.ceil()].into()
    }
}

/// 3D point type with `isize` members.
pub type Point3i = Point3<isize>;
