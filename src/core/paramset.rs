use std::collections;
use std::fmt;
use std::str::FromStr;

use core::geometry::{Normal3f, Point2f, Point3f, Vector2f, Vector3f};
use core::pbrt::Float;

#[derive(Clone, PartialEq)]
pub struct ParamList<T>(pub Vec<T>);

impl<T> From<Vec<T>> for ParamList<T> {
    fn from(vs: Vec<T>) -> Self {
        ParamList(vs)
    }
}

impl<T> fmt::Debug for ParamList<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ref v = self.0;
        if v.is_empty() {
            write!(f, "<>")?;
        }
        let mut it = v.iter();
        write!(f, "{:?}", it.next().unwrap())?;
        while let Some(i) = it.next() {
            write!(f, " {:?}", i)?;
        }
        Ok(())
    }
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

    pub fn find_one_string(&mut self, name: &str, default: &str) -> String {
        match self.find(name) {
            Some(Value::String(pl)) => pl.0.first().map_or(default.into(), |v| v.clone()),
            None => default.into(),
            _ => panic!("still working on it"),
        }
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
}

impl From<Vec<ParamSetItem>> for ParamSet {
    fn from(psis: Vec<ParamSetItem>) -> Self {
        let mut ps = ParamSet::new();
        for ref psi in psis.iter() {
            ps.add(&psi.name, psi.values.clone())
        }
        ps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_set() {
        let mut ps: ParamSet = vec![
            ParamSetItem::new("test0", Value::Float(vec![1., 2.].into())),
        ].into();
        assert_eq!(
            ps.find("test0").unwrap(),
            Value::Float(ParamList(vec![1., 2.]))
        );

        let mut ps: ParamSet = vec![
            ParamSetItem::new("test1", Value::Float(ParamList(vec![1., 2.]))),
        ].into();
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
        let mut ps: ParamSet = vec![
            ParamSetItem::new("test1", Value::Float(ParamList(vec![1., 2.]))),
            ParamSetItem::new(
                "test2",
                Value::String(ParamList(vec!["one".to_owned(), "two".to_owned()])),
            ),
            ParamSetItem::new("test3", Value::String(ParamList(vec![]))),
        ].into();

        let test2: String = "one".to_owned();
        assert_eq!(ps.find_one_string("test2", "one"), test2);

        // let test3: String = "one".to_owned();
        // assert_eq!(ps.find("test3").unwrap_or("one").first(), test3);
    }
}
