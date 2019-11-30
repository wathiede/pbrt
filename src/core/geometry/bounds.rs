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

use crate::core::geometry::point::Point2;
use crate::core::geometry::point::Point3;
use crate::core::geometry::Number;
use crate::Float;

/// Generic type for and 2D bounding boxes.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bounds2<T>
where
    T: Number,
{
    /// point representing the minimum x,y value of the bounds.
    pub p_min: Point2<T>,
    /// point representing the maximum x,y value of the bounds.
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

impl<T> From<([T; 2], [T; 2])> for Bounds2<T>
where
    T: Number,
{
    /// Create `Bounds2<T>` from tuple of slices.  It also ensures min/max are correct, regardless of
    /// how they're arranged in the incoming slices.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Bounds2f;
    /// use pbrt::core::geometry::Point2f;
    ///
    /// let b = Bounds2f::from(([2., 3.], [4., 5.]));
    /// assert_eq!(
    ///     b,
    ///     Bounds2f {
    ///         p_min: Point2f { x: 2., y: 3. },
    ///         p_max: Point2f { x: 4., y: 5. }
    ///     }
    /// );
    ///
    /// let b = Bounds2f::from(([5., 4.], [3., 2.]));
    /// assert_eq!(b, Bounds2f::from(([3., 2.], [5., 4.])));
    /// ```
    fn from((p1, p2): ([T; 2], [T; 2])) -> Self {
        let p1 = Point2::from(p1);
        let p2 = Point2::from(p2);
        (p1, p2).into()
    }
}

impl<T> From<(Point2<T>, Point2<T>)> for Bounds2<T>
where
    T: Number,
{
    /// Create `Bounds2<T>` from tuple of `Point2<t>`.  It also ensures min/max are correct, regardless of
    /// how they're arranged in the incoming `Point2<t>`.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Bounds2f;
    /// use pbrt::core::geometry::Point2f;
    ///
    /// let b = Bounds2f::from((Point2f::from([2., 3.]), Point2f::from([4., 5.])));
    /// assert_eq!(
    ///     b,
    ///     Bounds2f {
    ///         p_min: Point2f { x: 2., y: 3. },
    ///         p_max: Point2f { x: 4., y: 5. }
    ///     }
    /// );
    ///
    /// let b = Bounds2f::from((Point2f::from([5., 4.]), Point2f::from([3., 2.])));
    /// assert_eq!(b, Bounds2f::from(([3., 2.], [5., 4.])));
    /// ```
    fn from((p1, p2): (Point2<T>, Point2<T>)) -> Self {
        let p_min = Point2::from((
            if p1.x < p2.x { p1.x } else { p2.x },
            if p1.y < p2.y { p1.y } else { p2.y },
        ));
        let p_max = Point2::from((
            if p1.x > p2.x { p1.x } else { p2.x },
            if p1.y > p2.y { p1.y } else { p2.y },
        ));
        Bounds2 { p_min, p_max }
    }
}

impl<T> Bounds2<T>
where
    T: Number,
{
    /// Computes the area covered by this bounding box.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::geometry::Bounds2f;
    /// use pbrt::core::geometry::Point2f;
    ///
    /// let b = Bounds2f::from(([1., 1.], [3., 3.]));
    /// assert_eq!(b.area(), 4.);
    /// ```
    pub fn area(&self) -> T {
        let d = self.p_max - self.p_min;
        d.x * d.y
    }
}

/// 2D bounding box type with `Float` members.
pub type Bounds2f = Bounds2<Float>;

impl From<Bounds2i> for Bounds2f {
    fn from(b: Bounds2i) -> Self {
        Self {
            p_min: b.p_min.into(),
            p_max: b.p_max.into(),
        }
    }
}

/// 2D bounding box type with `isize` members.
pub type Bounds2i = Bounds2<isize>;

impl From<Bounds2f> for Bounds2i {
    fn from(b: Bounds2f) -> Self {
        Self {
            p_min: b.p_min.into(),
            p_max: b.p_max.into(),
        }
    }
}

/// Generic type for 3D bounding boxes.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Bounds3<T> {
    /// point representing the minimum x,y,z value of the bounds.
    pub p_min: Point3<T>,
    /// point representing the maximum x,y,z value of the bounds.
    pub p_max: Point3<T>,
}

/// 3D bounding box type with `Float` members.
pub type Bounds3f = Bounds3<Float>;
/// 3D bounding box type with `isize` members.
pub type Bounds3i = Bounds3<isize>;
