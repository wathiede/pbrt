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
use std::ops::Div;

use crate::core::geometry::Number;
use crate::Float;

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
