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
pub fn make_bool_param_set(name: &str, vals: Vec<bool>) -> ParamSet {
    vec![make_bool(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_bool(name: &str, vals: Vec<bool>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Bool(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
pub fn make_float_param_set(name: &str, vals: Vec<Float>) -> ParamSet {
    vec![make_float(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_float(name: &str, vals: Vec<Float>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Float(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
pub fn make_int_param_set(name: &str, vals: Vec<isize>) -> ParamSet {
    vec![make_int(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_int(name: &str, vals: Vec<isize>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Int(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
pub fn make_point2f_param_set(name: &str, vals: Vec<Point2f>) -> ParamSet {
    vec![make_point2f(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_point2f(name: &str, vals: Vec<Point2f>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Point2f(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
pub fn make_vector2f_param_set(name: &str, vals: Vec<Vector2f>) -> ParamSet {
    vec![make_vector2f(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_vector2f(name: &str, vals: Vec<Vector2f>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Vector2f(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
pub fn make_point3f_param_set(name: &str, vals: Vec<Point3f>) -> ParamSet {
    vec![make_point3f(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_point3f(name: &str, vals: Vec<Point3f>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Point3f(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
pub fn make_vector3f_param_set(name: &str, vals: Vec<Vector3f>) -> ParamSet {
    vec![make_vector3f(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_vector3f(name: &str, vals: Vec<Vector3f>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Vector3f(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
pub fn make_normal3f_param_set(name: &str, vals: Vec<Normal3f>) -> ParamSet {
    vec![make_normal3f(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_normal3f(name: &str, vals: Vec<Normal3f>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Normal3f(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
pub fn make_spectrum_param_set(name: &str, vals: Vec<Spectrum>) -> ParamSet {
    vec![make_spectrum(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_spectrum(name: &str, vals: Vec<Spectrum>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Spectrum(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
pub fn make_string_param_set(name: &str, vals: Vec<String>) -> ParamSet {
    vec![make_string(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_string(name: &str, vals: Vec<String>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::String(ParamList(vals)))
}

/// Creates a `ParamSet` with one entry containing `name` and set to `vals`.
pub fn make_texture_param_set(name: &str, vals: Vec<String>) -> ParamSet {
    vec![make_texture(name, vals)].into()
}

/// Creates a `ParamSetItem` with `name` set to `vals`.
pub fn make_texture(name: &str, vals: Vec<String>) -> ParamSetItem {
    ParamSetItem::new(name, &Value::Texture(ParamList(vals)))
}
