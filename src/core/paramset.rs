use std::collections;
use std::str::FromStr;

use core::pbrt::Float;

#[derive(Debug, Clone, PartialEq)]
struct ParamList<T>(Vec<T>);
// One idea, Vec<ParamList> where ParamList has a .first() method.

// TODO(wathiede): replace these types with imported proper types.
#[derive(Debug, Clone, PartialEq)]
struct Point2f(Float, Float);
#[derive(Debug, Clone, PartialEq)]
struct Vector2f(Float, Float);
#[derive(Debug, Clone, PartialEq)]
struct Point3f(Float, Float, Float);
#[derive(Debug, Clone, PartialEq)]
struct Vector3f(Float, Float, Float);
#[derive(Debug, Clone, PartialEq)]
struct Normal3f(Float, Float, Float);
#[derive(Debug, Clone, PartialEq)]
struct Spectrum(Float, Float, Float);

#[derive(Debug, Clone, PartialEq)]
enum Value {
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
}

struct ParamSetItem {
    name: String,
    values: Value,
}

struct ParamSet {
    values: collections::HashMap<String, ParamSetItem>,
    looked_up: collections::HashMap<String, bool>,
}

impl ParamSet {
    pub fn new() -> ParamSet {
        ParamSet {
            values: collections::HashMap::new(),
            looked_up: collections::HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, values: Value) {
        let name = String::from_str(name).unwrap();
        self.values.insert(
            name.clone(),
            ParamSetItem {
                name: name.clone(),
                values: values,
            },
        );
        self.looked_up.insert(name.clone(), false);
    }

    pub fn find(&self, name: &str) -> Option<Value> {
        // Defer unwrapping to call site or consider to use a macro.
        self.values.get(name).map(|psi| psi.values.clone())
    }

    // TODO(wathiede): remove or add helpers for all types.
    pub fn add_float(&mut self, name: &str, values: &[Float]) {
        let name = String::from_str(name).unwrap();
        self.values.insert(
            name.clone(),
            ParamSetItem {
                name: name.clone(),
                values: Value::Float(ParamList(values.to_vec())),
            },
        );
        self.looked_up.insert(name.clone(), false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_set() {
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
    }

}
