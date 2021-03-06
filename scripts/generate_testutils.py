# Copyright 2019 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
"""
Utility for generating helper functions for core::paramset::testutils
Update with:
    $ python generate_testutils.py > ../src/core/paramset/testutils.rs
"""

import collections

from paramset import input_types
from paramset import use_map

header = """
//! This module provides helpers for generating `ParamSet` structures concisely. This is useful
//! for doctests.

use crate::core::geometry::Normal3f;
use crate::core::geometry::Point2f;
use crate::core::geometry::Point3f;
use crate::core::geometry::Vector2f;
use crate::core::geometry::Vector3f;
use crate::core::paramset::ParamList;
use crate::core::paramset::ParamSet;
use crate::core::paramset::ParamSetItem;
use crate::core::paramset::Value;
use crate::core::spectrum::Spectrum;
use crate::Float;
"""

tmpl = """
/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
///
/// # Examples
/// ```
/// use pbrt::core::paramset::testutils::make_{2}_param_set;
/// {5}
/// let ps = make_{2}_param_set("value", vec![{3}]);
/// assert_eq!(ps.find_one_{2}("value", {4}), {3});
/// assert_eq!(ps.find_one_{2}("non-existent", {4}), {4});
/// ```
pub fn make_{2}_param_set(name: &str, vals: Vec<{1}>) -> ParamSet {{
    vec![make_{2}(name, vals)].into()
}}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_{2}(name: &str, vals: Vec<{1}>) -> ParamSetItem {{
    ParamSetItem::new(name, &Value::{0}(ParamList(vals)))
}}
"""

def gen():
    for t in input_types:
        use = ''
        if t.wrapped_type in use_map:
            use = '{}\n'.format(use_map[t.wrapped_type])

        print(tmpl.format(
            t.wrapped_type,
            t.native_type,
            t.wrapped_type.lower(),
            t.example_good,
            t.example_bad,
            use,
            ))

print(header)
gen()
