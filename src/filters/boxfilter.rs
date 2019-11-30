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
use crate::core::paramset::ParamSet;
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
    /// Create `BoxFilter` from `ParamSet`.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::filter::Filter;
    /// use pbrt::core::paramset::testutils::make_float_param_set;
    /// use pbrt::filters::boxfilter::BoxFilter;
    ///
    /// let ps = make_float_param_set("xwidth", vec![1.]);
    /// let bf = BoxFilter::create_box_filter(&ps);
    /// assert_eq!(bf.radius(), [1., 0.5].into());
    /// assert_eq!(bf.inv_radius(), [1., 2.].into());
    /// ```
    pub fn create_box_filter(ps: &ParamSet) -> Self {
        let xw = ps.find_one_float("xwidth", 0.5);
        let yw = ps.find_one_float("ywidth", 0.5);
        BoxFilter::new([xw, yw].into())
    }
}

impl Filter for BoxFilter {
    /// returns 1. for any point given.
    fn evaluate(&self, _: Point2f) -> Float {
        1.
    }
    /// return the radius this filter was created with.
    fn radius(&self) -> Vector2f {
        self.radius
    }
    /// return the inverse of the radius this filter was created with.
    fn inv_radius(&self) -> Vector2f {
        self.inv_radius
    }
}
