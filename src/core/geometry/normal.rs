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

use crate::{core::geometry::Number, Float};

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

impl<T> From<[T; 3]> for Normal3<T>
where
    T: Number,
{
    fn from(xyz: [T; 3]) -> Self {
        Normal3 {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        }
    }
}

impl<T> From<(T, T, T)> for Normal3<T>
where
    T: Number,
{
    fn from((x, y, z): (T, T, T)) -> Self {
        Normal3 { x, y, z }
    }
}

/// 3D normal type with `Float` members.
pub type Normal3f = Normal3<Float>;
