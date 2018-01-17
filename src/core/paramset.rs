use std::collections;
use std::str::FromStr;

use core::pbrt::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct ParamList<T>(pub Vec<T>);

// TODO(wathiede): replace these types with imported proper types.
#[derive(Debug, Clone, PartialEq)]
pub struct Point2f {
    pub x: Float,
    pub y: Float,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Vector2f {
    pub x: Float,
    pub y: Float,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Point3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Vector3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Normal3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Spectrum {
    pub x: Float,
    pub y: Float,
    pub z: Float,
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
    looked_up: bool,
}

impl ParamSetItem {
    pub fn new(name: &str, values: Value) -> ParamSetItem {
        ParamSetItem {
            name: String::from(name),
            values: values.clone(),
            looked_up: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParamSet {
    values: collections::HashMap<String, ParamSetItem>,
}

impl ParamSet {
    pub fn new() -> ParamSet {
        ParamSet {
            values: collections::HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, values: Value) {
        let name = String::from_str(name).unwrap();
        self.values.insert(
            name.clone(),
            ParamSetItem {
                name: name.clone(),
                values: values,
                looked_up: false,
            },
        );
    }

    pub fn find(&mut self, name: &str) -> Option<Value> {
        // Defer unwrapping to call site or consider to use a macro.
        self.values.get_mut(name).map(|psi| {
            psi.looked_up = true;
            psi.values.clone()
        })
    }

    pub fn report_unused(&self) -> bool {
        let mut unused = false;
        info!("report_unused");

        for (key, val) in self.values.iter() {
            if !val.looked_up {
                info!("* '{}' not used", key);
                unused = true
            }
        }

        unused
    }

    // TODO(wathiede): remove or add helpers for all types.
    pub fn add_float(&mut self, name: &str, values: &[Float]) {
        let name = String::from_str(name).unwrap();
        self.values.insert(
            name.clone(),
            ParamSetItem {
                name: name.clone(),
                values: Value::Float(ParamList(values.to_vec())),
                looked_up: false,
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate env_logger;

    #[test]
    fn test_param_set() {
        let _ = env_logger::init();
        let mut ps = ParamSet::new();
        ps.add_float("test1", &vec![1., 2.]);
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

}
