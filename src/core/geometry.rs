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
use std::ops::Div;

use crate::core::pbrt::{Float, Int};

pub trait Sqrt<RHS = Self> {
    type Output;
    fn sqrt(self) -> Self::Output;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

pub type Vector2f = Vector2<Float>;
pub type Vector2i = Vector2<Int>;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }
}

pub type Vector3f = Vector3<Float>;

// TODO(wathiede): Make this generic over float vs int.
impl Vector3f {
    pub fn normalize(&self) -> Vector3f {
        self / self.length()
    }

    pub fn length_squared(&self) -> Float {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }
}

impl From<[Float; 3]> for Vector3f {
    fn from(v: [Float; 3]) -> Self {
        Vector3f {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

// TODO(wathiede): Make this generic over float vs int.
impl<'a> Div<Float> for &'a Vector3f {
    type Output = Vector3f;

    fn div(self, rhs: Float) -> Vector3f {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

pub type Vector3i = Vector3<Int>;

impl Vector3i {
    pub fn normalize(&self) -> Vector3i {
        self / self.length()
    }

    pub fn length_squared(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z) as Float
    }

    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }
}

// TODO(wathiede): Make this generic over float vs int.
impl<'a> Div<Float> for &'a Vector3i {
    type Output = Vector3i;

    fn div(self, rhs: Float) -> Vector3i {
        Vector3 {
            x: (self.x as Float / rhs) as Int,
            y: (self.y as Float / rhs) as Int,
            z: (self.z as Float / rhs) as Int,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

pub type Point2f = Point2<Float>;
pub type Point2i = Point2<Int>;

#[derive(Debug, Clone, PartialEq)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Point3f = Point3<Float>;
pub type Point3i = Point3<Int>;

#[derive(Debug, Clone, PartialEq)]
pub struct Normal3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Normal3f = Normal3<Float>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        let v3f = Vector3f {
            x: 1.,
            y: 0.,
            z: 0.,
        };
        assert_eq!(v3f.length(), 1.);
        assert_eq!(
            v3f.normalize(),
            Vector3f {
                x: 1.,
                y: 0.,
                z: 0.,
            }
        );

        let v3f = Vector3f {
            x: 0.,
            y: 1.,
            z: 0.,
        };
        assert_eq!(v3f.length(), 1.);
        assert_eq!(
            v3f.normalize(),
            Vector3f {
                x: 0.,
                y: 1.,
                z: 0.,
            }
        );

        let v3f = Vector3f {
            x: 0.,
            y: 0.,
            z: 1.,
        };
        assert_eq!(v3f.length(), 1.);
        assert_eq!(
            v3f.normalize(),
            Vector3f {
                x: 0.,
                y: 0.,
                z: 1.,
            }
        );

        let v3i = Vector3i { x: 0, y: 0, z: 1 };
        assert_eq!(v3i.length(), 1.);
        assert_eq!(v3i.normalize(), Vector3i { x: 0, y: 0, z: 1 });
    }
}
