"""
Utility for generating helper functions for core::paramset::testutils
Update with:
    $ python generate_testutils.py > ../src/core/paramset/testutils.rs
"""

input_types = [
    ('Bool', 'bool'),
    ('Float', 'Float'),
    ('Int', 'isize'),
    ('Point2f', 'Point2f'),
    ('Vector2f', 'Vector2f'),
    ('Point3f', 'Point3f'),
    ('Vector3f', 'Vector3f'),
    ('Normal3f', 'Normal3f'),
    ('Spectrum', 'Spectrum'),
    ('String', 'String'),
    ('Texture', 'String'),
];

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
pub fn make_{2}_param_set(name: &str, vals: Vec<{1}>) -> ParamSet {{
    vec![make_{2}(name, vals)].into()
}}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_{2}(name: &str, vals: Vec<{1}>) -> ParamSetItem {{
    ParamSetItem::new(name, &Value::{0}(ParamList(vals)))
}}
"""

def gen():
    for enum, typ in input_types:
        print(tmpl.format(enum, typ, enum.lower()))

print(header)
gen()
