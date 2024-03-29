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

// TODO(wathiede): rethink ParamSet implement to more closely match C++ versions vectors of
// templated ParamSetItems.  Allow undocumented members until that rethink is done.
#![allow(missing_docs)]
//! Generic storage types created by parser and passed to factory functions when building a scene.

use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Formatter, Result},
    str::FromStr,
    sync::Arc,
};

use log::info;

use crate::{
    core::{
        geometry::{Normal3f, Point2f, Point3f, Vector2f, Vector3f},
        spectrum::Spectrum,
        texture::Texture,
    },
    Float,
};

pub mod testutils;

#[derive(Clone, PartialEq)]
pub struct ParamList<T>(pub Vec<T>);

impl<T> From<Vec<T>> for ParamList<T> {
    fn from(vs: Vec<T>) -> Self {
        ParamList(vs)
    }
}

impl<T> Debug for ParamList<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let v = &self.0;
        if v.is_empty() {
            write!(f, "<>")?;
        }
        let mut it = v.iter();
        write!(f, "<{:?}", it.next().unwrap())?;
        for i in it {
            write!(f, " {:?}", i)?;
        }
        write!(f, ">")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(ParamList<bool>),
    Float(ParamList<Float>),
    Int(ParamList<isize>),
    Point2f(ParamList<Point2f>),
    Vector2f(ParamList<Vector2f>),
    Point3f(ParamList<Point3f>),
    Vector3f(ParamList<Vector3f>),
    Normal3f(ParamList<Normal3f>),
    String(ParamList<String>),
    Texture(ParamList<String>),
    Spectrum(ParamList<Spectrum>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParamSetItem {
    pub name: String,
    pub values: Value,
    looked_up: RefCell<bool>,
}

impl ParamSetItem {
    /// Create a new `ParamSetItem` with the given `name` and `values`.
    pub fn new(name: &str, values: &Value) -> ParamSetItem {
        ParamSetItem {
            name: String::from(name),
            values: values.clone(),
            looked_up: RefCell::new(false),
        }
    }
}

/// `ParamSet` provides a generic way to pass data between the scene files and the factory
/// functions that create various pieces of the rendering pipeline.  It enables the renderer to be
/// extensible, many of the constructor methods on [PbrtAPI] take a name and a `ParamSet`.  This
/// allows new types of `Texture`s, `Camera`s, `Shape`s, etc. to be added without haven't to change
/// and method signatures.
///
/// [PbrtAPI]: crate::core::api::PbrtAPI
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParamSet {
    values: HashMap<String, ParamSetItem>,
}

// TODO(wathiede): try rewriting this using slice::chunks_exact()
fn iter3d(items: &[Float]) -> impl Iterator<Item = (Float, Float, Float)> + '_ {
    let xs = items
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if i % 3 == 0 { Some(v) } else { None });
    let ys = items
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if i % 3 == 1 { Some(v) } else { None });
    let zs = items
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if i % 3 == 2 { Some(v) } else { None });
    xs.zip(ys).zip(zs).map(|((x, y), z)| (x, y, z))
}

impl ParamSet {
    fn add(&mut self, name: &str, values: Value) {
        let name = String::from_str(name).unwrap();
        self.values.insert(
            name.clone(),
            ParamSetItem {
                name,
                values,
                looked_up: RefCell::new(false),
            },
        );
    }

    pub fn add_int(&mut self, name: &str, values: Vec<isize>) {
        self.add(name, Value::Int(ParamList(values)))
    }

    pub fn add_bool(&mut self, name: &str, values: Vec<bool>) {
        self.add(name, Value::Bool(ParamList(values)))
    }

    pub fn add_float(&mut self, name: &str, values: Vec<Float>) {
        self.add(name, Value::Float(ParamList(values)))
    }

    pub fn add_point2f(&mut self, name: &str, values: Vec<Point2f>) {
        self.add(name, Value::Point2f(ParamList(values)))
    }

    pub fn add_vector2f(&mut self, name: &str, values: Vec<Vector2f>) {
        self.add(name, Value::Vector2f(ParamList(values)))
    }

    pub fn add_point3f(&mut self, name: &str, values: Vec<Point3f>) {
        self.add(name, Value::Point3f(ParamList(values)))
    }

    pub fn add_vector3f(&mut self, name: &str, values: Vec<Vector3f>) {
        self.add(name, Value::Vector3f(ParamList(values)))
    }

    pub fn add_normal3f(&mut self, name: &str, values: Vec<Normal3f>) {
        self.add(name, Value::Normal3f(ParamList(values)))
    }

    pub fn add_rgb_spectrum(&mut self, name: &str, values: Vec<Float>) {
        assert_eq!(values.len() % 3, 0);
        let values = iter3d(&values)
            .map(|(r, g, b)| {
                let rgb: [Float; 3] = [r, g, b];
                Spectrum::from_rgb(rgb)
            })
            .collect();
        self.add(name, Value::Spectrum(ParamList(values)))
    }

    pub fn add_xyz_spectrum(&mut self, name: &str, values: Vec<Float>) {
        assert_eq!(values.len() % 3, 0);
        let values = iter3d(&values)
            .map(|(x, y, z)| {
                let xyz: [Float; 3] = [x, y, z];
                Spectrum::from_xyz(xyz)
            })
            .collect();
        self.add(name, Value::Spectrum(ParamList(values)))
    }

    pub fn add_blackbody(&mut self, _name: &str, _values: Vec<Float>) {
        todo!("core::paramset::Paramset::add_blackbody");
    }

    pub fn add_sampled_spectrum_files(&mut self, _name: &str, _values: Vec<String>) {
        todo!("core::paramset::Paramset::add_sampled_spectrum_files");
    }

    pub fn add_sampled_spectrum(&mut self, _name: &str, _values: Vec<Float>) {
        todo!("core::paramset::Paramset::add_sampled_spectrum");
    }

    pub fn add_string(&mut self, name: &str, values: Vec<String>) {
        self.add(name, Value::String(ParamList(values)))
    }

    pub fn add_texture(&mut self, name: &str, value: String) {
        self.add(name, Value::Texture(ParamList(vec![value])))
    }

    fn find(&self, name: &str) -> Option<Value> {
        // Defer unwrapping to call site or consider to use a macro.
        self.values.get(name).map(|psi| {
            *psi.looked_up.borrow_mut() = true;
            psi.values.clone()
        })
    }

    /// find_one_bool will return the first parameter in the set for the given
    /// `name`.  If no values are found `default` is returned. If the value by that
    /// name is found but isn't of type `bool` then `default` will be returned.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::paramset::testutils::make_bool_param_set;
    ///
    /// let ps = make_bool_param_set("value", vec![true]);
    /// assert_eq!(ps.find_one_bool("value", false), true);
    /// assert_eq!(ps.find_one_bool("non-existent", false), false);
    /// ```
    pub fn find_one_bool(&self, name: &str, default: bool) -> bool {
        match self.find(name) {
            Some(Value::Bool(pl)) => pl.0.first().map_or(default, |v| *v),
            None => default,
            _ => panic!("Unexpected type returned from find"),
        }
    }

    /// find_one_float will return the first parameter in the set for the given
    /// `name`.  If no values are found `default` is returned. If the value by that
    /// name is found but isn't of type `Float` then `default` will be returned.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::paramset::testutils::make_float_param_set;
    ///
    /// let ps = make_float_param_set("value", vec![1.]);
    /// assert_eq!(ps.find_one_float("value", 2.), 1.);
    /// assert_eq!(ps.find_one_float("non-existent", 2.), 2.);
    /// ```
    pub fn find_one_float(&self, name: &str, default: Float) -> Float {
        match self.find(name) {
            Some(Value::Float(pl)) => pl.0.first().map_or(default, |v| *v),
            None => default,
            _ => panic!("Unexpected type returned from find"),
        }
    }

    /// find_one_int will return the first parameter in the set for the given
    /// `name`.  If no values are found `default` is returned. If the value by that
    /// name is found but isn't of type `isize` then `default` will be returned.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::paramset::testutils::make_int_param_set;
    ///
    /// let ps = make_int_param_set("value", vec![1]);
    /// assert_eq!(ps.find_one_int("value", 2), 1);
    /// assert_eq!(ps.find_one_int("non-existent", 2), 2);
    /// ```
    pub fn find_one_int(&self, name: &str, default: isize) -> isize {
        match self.find(name) {
            Some(Value::Int(pl)) => pl.0.first().map_or(default, |v| *v),
            None => default,
            _ => panic!("Unexpected type returned from find"),
        }
    }

    /// find_one_point2f will return the first parameter in the set for the given
    /// `name`.  If no values are found `default` is returned. If the value by that
    /// name is found but isn't of type `Point2f` then `default` will be returned.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::{geometry::Point2f, paramset::testutils::make_point2f_param_set};
    ///
    /// let ps = make_point2f_param_set("value", vec![Point2f::from([1., 1.])]);
    /// assert_eq!(
    ///     ps.find_one_point2f("value", Point2f::from([2., 2.])),
    ///     Point2f::from([1., 1.])
    /// );
    /// assert_eq!(
    ///     ps.find_one_point2f("non-existent", Point2f::from([2., 2.])),
    ///     Point2f::from([2., 2.])
    /// );
    /// ```
    pub fn find_one_point2f(&self, name: &str, default: Point2f) -> Point2f {
        match self.find(name) {
            Some(Value::Point2f(pl)) => pl.0.first().map_or(default, |v| *v),
            None => default,
            _ => panic!("Unexpected type returned from find"),
        }
    }

    /// find_one_vector2f will return the first parameter in the set for the given
    /// `name`.  If no values are found `default` is returned. If the value by that
    /// name is found but isn't of type `Vector2f` then `default` will be returned.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::{geometry::Vector2f, paramset::testutils::make_vector2f_param_set};
    ///
    /// let ps = make_vector2f_param_set("value", vec![Vector2f::from([1., 1.])]);
    /// assert_eq!(
    ///     ps.find_one_vector2f("value", Vector2f::from([2., 2.])),
    ///     Vector2f::from([1., 1.])
    /// );
    /// assert_eq!(
    ///     ps.find_one_vector2f("non-existent", Vector2f::from([2., 2.])),
    ///     Vector2f::from([2., 2.])
    /// );
    /// ```
    pub fn find_one_vector2f(&self, name: &str, default: Vector2f) -> Vector2f {
        match self.find(name) {
            Some(Value::Vector2f(pl)) => pl.0.first().map_or(default, |v| *v),
            None => default,
            _ => panic!("Unexpected type returned from find"),
        }
    }

    /// find_one_point3f will return the first parameter in the set for the given
    /// `name`.  If no values are found `default` is returned. If the value by that
    /// name is found but isn't of type `Point3f` then `default` will be returned.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::{geometry::Point3f, paramset::testutils::make_point3f_param_set};
    ///
    /// let ps = make_point3f_param_set("value", vec![Point3f::from([1., 1., 1.])]);
    /// assert_eq!(
    ///     ps.find_one_point3f("value", Point3f::from([2., 2., 2.])),
    ///     Point3f::from([1., 1., 1.])
    /// );
    /// assert_eq!(
    ///     ps.find_one_point3f("non-existent", Point3f::from([2., 2., 2.])),
    ///     Point3f::from([2., 2., 2.])
    /// );
    /// ```
    pub fn find_one_point3f(&self, name: &str, default: Point3f) -> Point3f {
        match self.find(name) {
            Some(Value::Point3f(pl)) => pl.0.first().map_or(default, |v| *v),
            None => default,
            _ => panic!("Unexpected type returned from find"),
        }
    }

    /// find_one_vector3f will return the first parameter in the set for the given
    /// `name`.  If no values are found `default` is returned. If the value by that
    /// name is found but isn't of type `Vector3f` then `default` will be returned.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::{geometry::Vector3f, paramset::testutils::make_vector3f_param_set};
    ///
    /// let ps = make_vector3f_param_set("value", vec![Vector3f::from([1., 1., 1.])]);
    /// assert_eq!(
    ///     ps.find_one_vector3f("value", Vector3f::from([2., 2., 2.])),
    ///     Vector3f::from([1., 1., 1.])
    /// );
    /// assert_eq!(
    ///     ps.find_one_vector3f("non-existent", Vector3f::from([2., 2., 2.])),
    ///     Vector3f::from([2., 2., 2.])
    /// );
    /// ```
    pub fn find_one_vector3f(&self, name: &str, default: Vector3f) -> Vector3f {
        match self.find(name) {
            Some(Value::Vector3f(pl)) => pl.0.first().map_or(default, |v| *v),
            None => default,
            _ => panic!("Unexpected type returned from find"),
        }
    }

    /// find_one_normal3f will return the first parameter in the set for the given
    /// `name`.  If no values are found `default` is returned. If the value by that
    /// name is found but isn't of type `Normal3f` then `default` will be returned.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::{geometry::Normal3f, paramset::testutils::make_normal3f_param_set};
    ///
    /// let ps = make_normal3f_param_set("value", vec![Normal3f::from([1., 1., 1.])]);
    /// assert_eq!(
    ///     ps.find_one_normal3f("value", Normal3f::from([2., 2., 2.])),
    ///     Normal3f::from([1., 1., 1.])
    /// );
    /// assert_eq!(
    ///     ps.find_one_normal3f("non-existent", Normal3f::from([2., 2., 2.])),
    ///     Normal3f::from([2., 2., 2.])
    /// );
    /// ```
    pub fn find_one_normal3f(&self, name: &str, default: Normal3f) -> Normal3f {
        match self.find(name) {
            Some(Value::Normal3f(pl)) => pl.0.first().map_or(default, |v| *v),
            None => default,
            _ => panic!("Unexpected type returned from find"),
        }
    }

    /// find_one_spectrum will return the first parameter in the set for the given
    /// `name`.  If no values are found `default` is returned. If the value by that
    /// name is found but isn't of type `Spectrum` then `default` will be returned.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::{paramset::testutils::make_spectrum_param_set, spectrum::Spectrum};
    ///
    /// let ps = make_spectrum_param_set("value", vec![Spectrum::from_rgb([1., 1., 1.])]);
    /// assert_eq!(
    ///     ps.find_one_spectrum("value", Spectrum::from_rgb([2., 2., 2.])),
    ///     Spectrum::from_rgb([1., 1., 1.])
    /// );
    /// assert_eq!(
    ///     ps.find_one_spectrum("non-existent", Spectrum::from_rgb([2., 2., 2.])),
    ///     Spectrum::from_rgb([2., 2., 2.])
    /// );
    /// ```
    pub fn find_one_spectrum(&self, name: &str, default: Spectrum) -> Spectrum {
        match self.find(name) {
            Some(Value::Spectrum(pl)) => pl.0.first().map_or(default, |v| v.clone()),
            None => default,
            _ => panic!("Unexpected type returned from find"),
        }
    }

    /// find_one_string will return the first parameter in the set for the given
    /// `name`.  If no values are found `default` is returned. If the value by that
    /// name is found but isn't of type `String` then `default` will be returned.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::paramset::testutils::make_string_param_set;
    ///
    /// let ps = make_string_param_set("value", vec!["found".to_string()]);
    /// assert_eq!(ps.find_one_string("value", "default"), "found".to_string());
    /// assert_eq!(
    ///     ps.find_one_string("non-existent", "default"),
    ///     "default".to_string()
    /// );
    /// ```
    pub fn find_one_string(&self, name: &str, default: &str) -> String {
        match self.find(name) {
            Some(Value::String(pl)) => pl.0.first().map_or(default.to_string(), |v| v.clone()),
            None => default.to_string(),
            _ => panic!("Unexpected type returned from find"),
        }
    }

    /// find_one_filename will return the first parameter in the set for the given
    /// `name`.  If no values are found `default` is returned. If the value by that
    /// name is found but isn't of type `Filename` then `default` will be returned.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::paramset::testutils::make_filename_param_set;
    ///
    /// let ps = make_filename_param_set("value", vec!["found".to_string()]);
    /// assert_eq!(
    ///     ps.find_one_filename("value", "default"),
    ///     "found".to_string()
    /// );
    /// assert_eq!(
    ///     ps.find_one_filename("non-existent", "default"),
    ///     "default".to_string()
    /// );
    /// ```
    pub fn find_one_filename(&self, name: &str, default: &str) -> String {
        let filename = self.find_one_string(name, "");
        if filename.is_empty() {
            return default.to_string();
        }
        // TODO(wathiede): need to create equivalents for AbsolutePath and ResolveFilename and
        // wrap `filename` in them.
        filename
    }

    /// find_one_texture will return the first parameter in the set for the given
    /// `name`.  If no values are found `default` is returned. If the value by that
    /// name is found but isn't of type `String` then `default` will be returned.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::paramset::testutils::make_texture_param_set;
    ///
    /// let ps = make_texture_param_set("value", vec!["found".to_string()]);
    /// assert_eq!(ps.find_one_texture("value", "default"), "found".to_string());
    /// assert_eq!(
    ///     ps.find_one_texture("non-existent", "default"),
    ///     "default".to_string()
    /// );
    /// ```
    pub fn find_one_texture(&self, name: &str, default: &str) -> String {
        match self.find(name) {
            Some(Value::Texture(pl)) => pl.0.first().map_or(default.to_string(), |v| v.clone()),
            None => default.to_string(),
            _ => panic!("Unexpected type returned from find"),
        }
    }

    /// `report_unused` will print out all values in this `ParamSet` that have not been accessed,
    /// will return true if any unused values are found.
    /// Useful after parsing a scene to see what configuration data was superfluous, or for
    /// detecting incomplete implementations of scene factory fuctions.
    pub fn report_unused(&self) -> bool {
        let mut unused = false;
        info!("report_unused");

        for (key, val) in &self.values {
            if !(*val.looked_up.borrow()) {
                info!("* '{}' not used", key);
                unused = true
            }
        }

        unused
    }
}

impl From<Vec<ParamSetItem>> for ParamSet {
    fn from(psis: Vec<ParamSetItem>) -> Self {
        let mut ps: ParamSet = Default::default();
        for psi in &psis {
            ps.add(&psi.name, psi.values.clone())
        }
        ps
    }
}

/// `TextureParams` represent values necessary to create a new [Texture].
/// TODO(wathiede): currently only a stub, textures not implemented.
///
/// [Texture]: crate::core::texture::Texture
#[derive(Default)]
pub struct TextureParams {
    _float_textures: HashMap<String, Arc<dyn Texture<Float>>>,
    _specturm_textures: HashMap<String, Arc<dyn Texture<Spectrum>>>,
    geom_params: ParamSet,
    material_params: ParamSet,
}

impl TextureParams {
    /// Create a new `TextureParams` from the given set of parameters.
    /// TODO(wathiede): currently only a stub, textures not implemented.
    pub fn new(
        geom_params: ParamSet,
        material_params: ParamSet,
        float_textures: HashMap<String, Arc<dyn Texture<Float>>>,
        specturm_textures: HashMap<String, Arc<dyn Texture<Spectrum>>>,
    ) -> TextureParams {
        TextureParams {
            _float_textures: float_textures,
            _specturm_textures: specturm_textures,
            geom_params,
            material_params,
        }
    }

    /// find_float will return the first `Float` value with the given `name` in this
    /// `TextureParams`'s `geom_params` set, if none is found, it will find the first `Float` value
    /// in the `material_params` set.  If no value is found there, the provided `default` will be
    /// returned.
    pub fn find_float(&self, name: &str, default: Float) -> Float {
        self.geom_params
            .find_one_float(name, self.material_params.find_one_float(name, default))
    }

    /// find_spectrum will return the first `Spectrum` value with the given `name` in this
    /// `TextureParams`'s `geom_params` set, if none is found, it will find the first `Spectrum`
    /// value in the `material_params` set.  If no value is found there, the provided `default`
    /// will be returned.
    pub fn find_spectrum(&self, name: &str, default: Spectrum) -> Spectrum {
        self.geom_params
            .find_one_spectrum(name, self.material_params.find_one_spectrum(name, default))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_set() {
        let ps: ParamSet = vec![ParamSetItem::new(
            "test0",
            &Value::Float(vec![1., 2.].into()),
        )]
        .into();
        assert_eq!(
            ps.find("test0").unwrap(),
            Value::Float(ParamList(vec![1., 2.]))
        );

        let mut ps: ParamSet = vec![ParamSetItem::new(
            "test1",
            &Value::Float(ParamList(vec![1., 2.])),
        )]
        .into();
        assert_eq!(
            ps.find("test1").unwrap(),
            Value::Float(ParamList(vec![1., 2.]))
        );

        assert_eq!(
            ps.find("notfound")
                .unwrap_or(Value::Float(ParamList(vec![1., 2.]))),
            Value::Float(ParamList(vec![1., 2.]))
        );

        ps.add("test2", Value::Float(ParamList(vec![3., 4.])));
        assert_eq!(
            ps.find("test2").unwrap(),
            Value::Float(ParamList(vec![3., 4.]))
        );
        assert_eq!(ps.find("test3"), None);
        ps.add("bools", Value::Bool(ParamList(vec![true, true, false])));
        assert_eq!(
            ps.find("bools").unwrap(),
            Value::Bool(ParamList(vec![true, true, false]))
        );
        assert_eq!(ps.find("test3"), None);

        assert!(!ps.report_unused());

        ps.add("notused", Value::Bool(ParamList(vec![true, true, false])));
        assert!(ps.report_unused());
    }

    #[test]
    fn test_param_set_find() {
        let ps: ParamSet = vec![
            ParamSetItem::new("test1", &Value::Float(ParamList(vec![1., 2.]))),
            ParamSetItem::new(
                "test2",
                &Value::String(ParamList(vec!["one".to_owned(), "two".to_owned()])),
            ),
            ParamSetItem::new("test3", &Value::String(ParamList(vec![]))),
        ]
        .into();

        let test2: String = "one".to_owned();
        assert_eq!(ps.find_one_string("test2", "one"), test2);

        // let test3: String = "one".to_owned();
        // assert_eq!(ps.find("test3").unwrap_or("one").first(), test3);
    }
}
