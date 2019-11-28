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
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::str::FromStr;
use std::sync::Arc;

use log::info;

use crate::core::geometry::{Normal3f, Point2f, Point3f, Vector2f, Vector3f};
use crate::core::pbrt::Float;
use crate::core::spectrum::Spectrum;
use crate::core::texture::Texture;

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
    Int(ParamList<i64>),
    Point2f(ParamList<Point2f>),
    Vector2f(ParamList<Vector2f>),
    Point3f(ParamList<Point3f>),
    Vector3f(ParamList<Vector3f>),
    Normal3f(ParamList<Normal3f>),
    Spectrum(ParamList<Spectrum>),
    String(ParamList<String>),
    Texture(ParamList<String>),
    // TODO(wathiede): make a generic 'Spectrum' type?
    RGB(ParamList<Float>),
    Blackbody(ParamList<Float>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParamSetItem {
    pub name: String,
    pub values: Value,
    looked_up: RefCell<bool>,
}

impl ParamSetItem {
    pub fn new(name: &str, values: &Value) -> ParamSetItem {
        ParamSetItem {
            name: String::from(name),
            values: values.clone(),
            looked_up: RefCell::new(false),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParamSet {
    values: HashMap<String, ParamSetItem>,
}

impl ParamSet {
    pub fn add(&mut self, name: &str, values: Value) {
        let name = String::from_str(name).unwrap();
        self.values.insert(
            name.clone(),
            ParamSetItem {
                name: name.clone(),
                values,
                looked_up: RefCell::new(false),
            },
        );
    }

    pub fn find(&self, name: &str) -> Option<Value> {
        // Defer unwrapping to call site or consider to use a macro.
        self.values.get(name).map(|psi| {
            *psi.looked_up.borrow_mut() = true;
            psi.values.clone()
        })
    }

    pub fn find_one_float(&self, name: &str, default: Float) -> Float {
        match self.find(name) {
            Some(Value::Float(pl)) => pl.0.first().map_or(default.into(), |v| v.clone()),
            None => default.into(),
            _ => panic!("Unexpected type returned from find"),
        }
    }

    pub fn find_one_string(&self, name: &str, default: &str) -> String {
        match self.find(name) {
            Some(Value::String(pl)) => pl.0.first().map_or(default.into(), |v| v.clone()),
            None => default.into(),
            _ => panic!("Unexpected type returned from find"),
        }
    }

    pub fn find_one_spectrum(&self, name: &str, default: Spectrum) -> Spectrum {
        match self.find(name) {
            Some(Value::Spectrum(pl)) => pl.0.first().map_or(default.into(), |v| v.clone()),
            None => default.into(),
            _ => panic!("Unexpected type returned from find"),
        }
    }

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

#[derive(Default)]
pub struct TextureParams {
    _float_textures: HashMap<String, Arc<dyn Texture<Float>>>,
    _specturm_textures: HashMap<String, Arc<dyn Texture<Spectrum>>>,
    geom_params: ParamSet,
    material_params: ParamSet,
}

impl TextureParams {
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

    pub fn find_float(&self, name: &str, default: Float) -> Float {
        self.geom_params
            .find_one_float(name, self.material_params.find_one_float(name, default))
    }

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
