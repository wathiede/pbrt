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
Utility for generating a fragment for core::paramset::ParamSet
Update with:
    $ python generate_paramset_frag.py > /tmp/frag.rs
    # Manually merge into ParamSet.
"""

import collections

from paramset import input_types
from paramset import use_map

tmpl = """
/// find_one_{2} will return the first parameter in the set for the given `name.  If no values are found `default` is returned.
///
/// # Examples
/// ```
/// use pbrt::core::paramset::testutils::make_{2}_param_set;
/// {5}
/// let ps = make_{2}_param_set("value", vec![{3}]);
/// assert_eq!(ps.find_one_{2}("value", {4}), {3});
/// assert_eq!(ps.find_one_{2}("non-existent", {4}), {4});
/// ```
pub fn find_one_{2}(&self, name: &str, default: {1}) -> {1} {{
    match self.find(name) {{
        Some(Value::{0}(pl)) => pl.0.first().map_or(default, |v| v.clone()),
        None => default,
        _ => panic!("Unexpected type returned from find"),
    }}
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

gen()
