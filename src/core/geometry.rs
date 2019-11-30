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

/// Trait for ensuring methods present on only `{float}` or `{integer}` types have appropriate
/// implementations as necessary for this crate.
pub trait Number
where
    Self: std::marker::Sized
        + Copy
        + fmt::Display
        + Add
        + Add<Output = Self>
        + Div
        + Div<Output = Self>
        + Mul
        + Mul<Output = Self>
        + Sub
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

/// Generic type for any 2D vector.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vector2<T> {
    /// The x coordinate of this vector.
    pub x: T,
    /// The y coordinate of this vector.
    pub y: T,
}

/// 2D vector type with `Float` members.
pub type Vector2f = Vector2<Float>;
/// 2D vector type with `isize` members.
pub type Vector2i = Vector2<isize>;

/// Generic type for any 3D vector.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vector3<T>
where
    T: Number,
{
    /// The x coordinate of this vector.
    pub x: T,
    /// The y coordinate of this vector.
    pub y: T,
    /// The z coordinate of this vector.
    pub z: T,
}

impl<T> Vector3<T>
where
    T: Number,
{
    /// Create a new `Vector3` with the given x,y,z values.
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }

    fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }
}

impl<T> From<[T; 3]> for Vector3<T>
where
    T: Number,
{
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Vector3f;
    /// assert_eq!(Vector3f::from([1., 2., 3.]), Vector3f::new(1., 2., 3.));
    /// ```
    /// ```should_panic
    /// use pbrt;
    /// use pbrt::core::geometry::Vector3f;
    /// use pbrt::Float;
    /// let v = Vector3f::from([pbrt::float::NAN, 2., 3.]);
    /// ```
    fn from(v: [T; 3]) -> Self {
        let v = Self {
            x: v[0],
            y: v[1],
            z: v[2],
        };
        debug_assert!(!v.has_nans());
        v
    }
}

/// 3D vector type with `Float` members.
pub type Vector3f = Vector3<Float>;

// TODO(wathiede): Make this generic over float vs int.
impl Vector3f {
    /// Compute a unit vector form self.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Vector3f;
    ///
    /// let v: Vector3f = [1., 0., 0.].into();
    /// assert_eq!(v.normalize(), [1., 0., 0.].into());
    ///
    /// let v: Vector3f = [0., 1., 0.].into();
    /// assert_eq!(v.normalize(), [0., 1., 0.].into());
    ///
    /// let v: Vector3f = [0., 0., 1.].into();
    /// assert_eq!(v.normalize(), [0., 0., 1.].into());
    /// ```
    pub fn normalize(&self) -> Vector3f {
        self / self.length()
    }

    /// Compute the squared length of the `Vector3f`.  This saves a sqrt over length, and is
    /// useful if you just want to compare to `Vector3f`s lengths, and don't need the actual value.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Vector3f;
    ///
    /// let v: Vector3f = [1., 0., 0.].into();
    /// assert_eq!(v.length_squared(), 1.);
    ///
    /// let v: Vector3f = [2., 0., 0.].into();
    /// assert_eq!(v.length_squared(), 4.);
    /// ```
    pub fn length_squared(&self) -> Float {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Compute the length of the `Vector3f`.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Vector3f;
    ///
    /// let v: Vector3f = [1., 0., 0.].into();
    /// assert_eq!(v.length(), 1.);
    ///
    /// let v: Vector3f = [0., 1., 0.].into();
    /// assert_eq!(v.length(), 1.);
    ///
    /// let v: Vector3f = [0., 0., 1.].into();
    /// assert_eq!(v.length(), 1.);
    ///
    /// let v: Vector3f = [2., 0., 0.].into();
    /// assert_eq!(v.length(), 2.);
    /// ```
    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }
}

// TODO(wathiede): Make this generic over float vs int.
impl<'a> Div<Float> for &'a Vector3f {
    type Output = Vector3f;

    fn div(self, rhs: Float) -> Vector3f {
        debug_assert!(!rhs.is_nan());
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

/// 2D vector type with `isize` members.
pub type Vector3i = Vector3<isize>;

impl Vector3i {
    /// Compute a unit vector form self.
    pub fn normalize(&self) -> Vector3i {
        self / self.length()
    }

    /// Compute the squared length of the `Vector3i`.  This saves a sqrt over length, and is
    /// useful if you just want to compare to `Vector3i`s lengths, and don't need the actual value.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Vector3i;
    ///
    /// let v: Vector3i = [1, 0, 0].into();
    /// assert_eq!(v.length_squared(), 1.);
    ///
    /// let v: Vector3i = [2, 0, 0].into();
    /// assert_eq!(v.length_squared(), 4.);
    /// ```
    pub fn length_squared(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z) as Float
    }

    /// Compute the length of the `Vector3i`.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Vector3i;
    ///
    /// let v: Vector3i = [1, 0, 0].into();
    /// assert_eq!(v.length(), 1.);
    ///
    /// let v: Vector3i = [2, 0, 0].into();
    /// assert_eq!(v.length(), 2.);
    /// ```
    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }
}

// TODO(wathiede): Make this generic over float vs int.
impl<'a> Div<Float> for &'a Vector3i {
    type Output = Vector3i;

    fn div(self, rhs: Float) -> Vector3i {
        debug_assert!(!rhs.is_nan());
        Vector3 {
            x: (self.x as Float / rhs) as isize,
            y: (self.y as Float / rhs) as isize,
            z: (self.z as Float / rhs) as isize,
        }
    }
}

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

/// Generic type for any 3D normal.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Normal3<T> {
    /// The x coordinate of this normal.
    pub x: T,
    /// The y coordinate of this normal.
    pub y: T,
    /// The z coordinate of this normal.
    pub z: T,
}

/// 3D normal type with `Float` members.
pub type Normal3f = Normal3<Float>;

/// Generic type for 2D bounding boxes.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bounds2<T>
where
    T: Number,
{
    /// point representing the minimum x,y value of the bounds.
    pub p_min: Point2<T>,
    /// point representing the maxium x,y value of the bounds.
    pub p_max: Point2<T>,
}

impl<T> Default for Bounds2<T>
where
    T: Number,
{
    fn default() -> Self {
        Self {
            p_min: Point2 {
                x: T::max_value(),
                y: T::max_value(),
            },
            p_max: Point2 {
                x: T::min_value(),
                y: T::min_value(),
            },
        }
    }
}

impl<T> fmt::Display for Bounds2<T>
where
    T: Number,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {} - {} ]", self.p_min, self.p_max)
    }
}

impl<T> From<(Point2<T>, Point2<T>)> for Bounds2<T>
where
    T: Number,
{
    fn from((p_min, p_max): (Point2<T>, Point2<T>)) -> Self {
        Bounds2 { p_min, p_max }
    }
}

impl<T> Bounds2<T>
where
    T: Number,
{
    /// Computes the areas covered by this bound.
    pub fn area(&self) -> T {
        let d = self.p_max - self.p_min;
        d.x * d.y
    }
}

/// 2D bounding box type with `Float` members.
pub type Bounds2f = Bounds2<Float>;
/// 2D bounding box type with `isize` members.
pub type Bounds2i = Bounds2<isize>;

/// Generic type for 3D bounding boxes.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Bounds3<T> {
    /// point representing the minimum x,y,z value of the bounds.
    pub p_min: Point3<T>,
    /// point representing the maxium x,y,z value of the bounds.
    pub p_max: Point3<T>,
}

/// 3D bounding box type with `Float` members.
pub type Bounds3f = Bounds3<Float>;
/// 3D bounding box type with `isize` members.
pub type Bounds3i = Bounds3<isize>;
