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

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
///
/// # Examples
/// ```
/// use pbrt::core::paramset::testutils::make_bool_param_set;
///
/// let ps = make_bool_param_set("value", vec![true]);
/// assert_eq!(ps.find_one_bool("value", false), true);
/// assert_eq!(ps.find_one_bool("non-existent", false), false);
/// ```
pub fn make_bool_param_set(name: &str, vals: Vec<bool>) -> ParamSet {
    vec![make_bool(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_bool(name: &str, vals: Vec<bool>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Bool(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
///
/// # Examples
/// ```
/// use pbrt::core::paramset::testutils::make_float_param_set;
///
/// let ps = make_float_param_set("value", vec![1.]);
/// assert_eq!(ps.find_one_float("value", 2.), 1.);
/// assert_eq!(ps.find_one_float("non-existent", 2.), 2.);
/// ```
pub fn make_float_param_set(name: &str, vals: Vec<Float>) -> ParamSet {
    vec![make_float(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_float(name: &str, vals: Vec<Float>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Float(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
///
/// # Examples
/// ```
/// use pbrt::core::paramset::testutils::make_int_param_set;
///
/// let ps = make_int_param_set("value", vec![1]);
/// assert_eq!(ps.find_one_int("value", 2), 1);
/// assert_eq!(ps.find_one_int("non-existent", 2), 2);
/// ```
pub fn make_int_param_set(name: &str, vals: Vec<isize>) -> ParamSet {
    vec![make_int(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_int(name: &str, vals: Vec<isize>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Int(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
///
/// # Examples
/// ```
/// use pbrt::core::paramset::testutils::make_point2f_param_set;
/// use pbrt::core::geometry::Point2f;

/// let ps = make_point2f_param_set("value", vec![Point2f::from([1., 1.])]);
/// assert_eq!(ps.find_one_point2f("value", Point2f::from([2., 2.])), Point2f::from([1., 1.]));
/// assert_eq!(ps.find_one_point2f("non-existent", Point2f::from([2., 2.])), Point2f::from([2., 2.]));
/// ```
pub fn make_point2f_param_set(name: &str, vals: Vec<Point2f>) -> ParamSet {
    vec![make_point2f(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_point2f(name: &str, vals: Vec<Point2f>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Point2f(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
///
/// # Examples
/// ```
/// use pbrt::core::paramset::testutils::make_vector2f_param_set;
/// use pbrt::core::geometry::Vector2f;

/// let ps = make_vector2f_param_set("value", vec![Vector2f::from([1., 1.])]);
/// assert_eq!(ps.find_one_vector2f("value", Vector2f::from([2., 2.])), Vector2f::from([1., 1.]));
/// assert_eq!(ps.find_one_vector2f("non-existent", Vector2f::from([2., 2.])), Vector2f::from([2., 2.]));
/// ```
pub fn make_vector2f_param_set(name: &str, vals: Vec<Vector2f>) -> ParamSet {
    vec![make_vector2f(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_vector2f(name: &str, vals: Vec<Vector2f>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Vector2f(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
///
/// # Examples
/// ```
/// use pbrt::core::paramset::testutils::make_point3f_param_set;
/// use pbrt::core::geometry::Point3f;

/// let ps = make_point3f_param_set("value", vec![Point3f::from([1., 1., 1.])]);
/// assert_eq!(ps.find_one_point3f("value", Point3f::from([2., 2., 2.])), Point3f::from([1., 1., 1.]));
/// assert_eq!(ps.find_one_point3f("non-existent", Point3f::from([2., 2., 2.])), Point3f::from([2., 2., 2.]));
/// ```
pub fn make_point3f_param_set(name: &str, vals: Vec<Point3f>) -> ParamSet {
    vec![make_point3f(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_point3f(name: &str, vals: Vec<Point3f>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Point3f(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
///
/// # Examples
/// ```
/// use pbrt::core::paramset::testutils::make_vector3f_param_set;
/// use pbrt::core::geometry::Vector3f;

/// let ps = make_vector3f_param_set("value", vec![Vector3f::from([1., 1., 1.])]);
/// assert_eq!(ps.find_one_vector3f("value", Vector3f::from([2., 2., 2.])), Vector3f::from([1., 1., 1.]));
/// assert_eq!(ps.find_one_vector3f("non-existent", Vector3f::from([2., 2., 2.])), Vector3f::from([2., 2., 2.]));
/// ```
pub fn make_vector3f_param_set(name: &str, vals: Vec<Vector3f>) -> ParamSet {
    vec![make_vector3f(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_vector3f(name: &str, vals: Vec<Vector3f>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Vector3f(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
///
/// # Examples
/// ```
/// use pbrt::core::paramset::testutils::make_normal3f_param_set;
/// use pbrt::core::geometry::Normal3f;

/// let ps = make_normal3f_param_set("value", vec![Normal3f::from([1., 1., 1.])]);
/// assert_eq!(ps.find_one_normal3f("value", Normal3f::from([2., 2., 2.])), Normal3f::from([1., 1., 1.]));
/// assert_eq!(ps.find_one_normal3f("non-existent", Normal3f::from([2., 2., 2.])), Normal3f::from([2., 2., 2.]));
/// ```
pub fn make_normal3f_param_set(name: &str, vals: Vec<Normal3f>) -> ParamSet {
    vec![make_normal3f(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_normal3f(name: &str, vals: Vec<Normal3f>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Normal3f(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
///
/// # Examples
/// ```
/// use pbrt::core::paramset::testutils::make_spectrum_param_set;
/// use pbrt::core::spectrum::Spectrum;

/// let ps = make_spectrum_param_set("value", vec![Spectrum::from_rgb([1., 1., 1.])]);
/// assert_eq!(ps.find_one_spectrum("value", Spectrum::from_rgb([2., 2., 2.])), Spectrum::from_rgb([1., 1., 1.]));
/// assert_eq!(ps.find_one_spectrum("non-existent", Spectrum::from_rgb([2., 2., 2.])), Spectrum::from_rgb([2., 2., 2.]));
/// ```
pub fn make_spectrum_param_set(name: &str, vals: Vec<Spectrum>) -> ParamSet {
    vec![make_spectrum(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_spectrum(name: &str, vals: Vec<Spectrum>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Spectrum(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
///
/// # Examples
/// ```
/// use pbrt::core::paramset::testutils::make_string_param_set;
///
/// let ps = make_string_param_set("value", vec!["found".to_string()]);
/// assert_eq!(
///     ps.find_one_string("value", "default".to_string()),
///     "found".to_string()
/// );
/// assert_eq!(
///     ps.find_one_string("non-existent", "default".to_string()),
///     "default".to_string()
/// );
/// ```
pub fn make_string_param_set(name: &str, vals: Vec<String>) -> ParamSet {
    vec![make_string(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_string(name: &str, vals: Vec<String>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::String(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
///
/// # Examples
/// ```
/// use pbrt::core::paramset::testutils::make_texture_param_set;
///
/// let ps = make_texture_param_set("value", vec!["found".to_string()]);
/// assert_eq!(
///     ps.find_one_texture("value", "default".to_string()),
///     "found".to_string()
/// );
/// assert_eq!(
///     ps.find_one_texture("non-existent", "default".to_string()),
///     "default".to_string()
/// );
/// ```
pub fn make_texture_param_set(name: &str, vals: Vec<String>) -> ParamSet {
    vec![make_texture(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_texture(name: &str, vals: Vec<String>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Texture(ParamList(vals)))
}
