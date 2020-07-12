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
//use std::str;
//use std::str::FromStr;

use thiserror::Error;

//use crate::core::geometry::Point3f;
use crate::core::paramset::ParamSet;
//use crate::core::paramset::{ParamList, ParamSet, ParamSetItem, Value};
use crate::Float;

#[derive(PartialEq, Debug, Error)]
pub enum Error {
    #[error("input not utf-8")]
    StrError(#[from] std::str::Utf8Error),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Directive {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    // TODO(wathiede): convert to 3 x Vector3f?
    LookAt(
        Float, Float, Float, // eye xyz
        Float, Float, Float, // look xyz
        Float, Float, Float, // up xyz
    ),
    Camera(String, ParamSet),
    Sampler(String, ParamSet),
    Integrator(String, ParamSet),
    Film(String, ParamSet),
    WorldBegin,
    WorldEnd,
    AttributeBegin,
    AttributeEnd,
    LightSource(String, ParamSet),
    Material(String, ParamSet),
    Shape(String, ParamSet),
    Translate(Float, Float, Float),
    Scale(Float, Float, Float),
    Rotate(Float, Float, Float, Float),
    Texture(
        String, // name
        String, // type
        String, // texname
        ParamSet,
    ),
    Unhandled(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    pub directives: Vec<Directive>,
}

pub fn parse_scene(input: &[u8]) -> Result<Scene, Error> {
    let pairs = PbrtParser::parse(Rule::file, &std::str::from_utf8(input)?)?;
    Ok(Scene::from(pairs))
}

#[derive(Parser)]
#[grammar = "core/pbrt-parser.pest"]
struct PbrtParser;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_param_set_item_float() -> Result<()> {
        PbrtParser::parse(Rule::param_set_item, r#""float foo" [ 0 1 2 3 4 5 ]"#)?;
        Ok(())
    }
    #[test]
    fn test_param_set_item_color() -> Result<()> {
        PbrtParser::parse(Rule::param_set_item, r#""color foo" [ 0 1 2 3 4 5 ]"#)?;
        Ok(())
    }
    /*
    #[test]
    fn test_param_set_item_integer() -> Result<()> {
        PbrtParser::parse(Rule::param_set_item, r#""integer foo" [ 0 1 2 3 4 5 ]"#)?;
        Ok(())
    }
    */
}
