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

//! Utilities for parsing pbrt scene files.
use std::convert::TryFrom;

use log::{error, warn};
use thiserror::Error;

use crate::core::api::API;
use crate::core::geometry::{Normal3f, Point2f, Point3f, Vector2f, Vector3f};
use crate::core::paramset::ParamSet;
use crate::Float;

/// Error type for tokenization and parsing errors.
#[derive(PartialEq, Debug, Error)]
pub enum Error {
    /// Input data isn't valid utf-8.
    #[error("input not utf-8")]
    StrError(#[from] std::str::Utf8Error),
    /// Input isn't a valid number.
    #[error("input not float")]
    NumberErr(#[from] std::num::ParseFloatError),
    /// Quoted string without closing quote.
    #[error("unterminated string")]
    UnterminatedString,
    /// Hit end-of-file unexpectedly while parsing.
    #[error("premature EOF")]
    EOF,
    /// Unknown token resulting in invalid syntax.
    #[error("syntax error: '{0}'")]
    Syntax(String),
    /// Attempt to unquote a string that was not quoted.
    #[error("expected quoted string")]
    Unquoted(String),
    /// Mixed string and numeric parameters found.
    #[error("mixed string and numeric parameters")]
    MixedParameters,
    /// Hit a part of the parser not yet implemented.
    // TODO(wathiede): remove this when Parser::parse() is complete.
    #[error("have not yet implemented '{0}'")]
    NotImplemented(String),
}

/// Tokenizer holds state necessary to tokenize a pbrt scene file.
pub struct Tokenizer<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<&'a str, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let token_start = self.pos;
            match self.get_byte() {
                // EOF
                None => return None,
                Some(b' ') | Some(b'\n') | Some(b'\t') | Some(b'\r') => (),
                Some(b'"') => {
                    // scan to closing quote
                    let mut have_escaped = false;
                    loop {
                        match self.get_byte() {
                            Some(byte) if byte == b'"' => break,
                            None => return Some(Err(Error::EOF)),
                            Some(b'\n') => return Some(Err(Error::UnterminatedString)),
                            Some(b'\\') => {
                                have_escaped = true;
                                if let None = self.get_byte() {
                                    return Some(Err(Error::EOF));
                                }
                            }
                            _ => (),
                        }
                    }

                    if !have_escaped {
                        return self.token(token_start);
                    } else {
                        unimplemented!();
                        /*
                        sEscaped.clear();
                        for (const char *p = tokenStart; p < pos; ++p) {
                            if (*p != '\\')
                                sEscaped.push_back(*p);
                            else {
                                ++p;
                                CHECK_LT(p, pos);
                                sEscaped.push_back(decodeEscaped(*p));
                            }
                        }
                        return {sEscaped.data(), sEscaped.size()};
                        */
                    }
                }

                Some(b'[') | Some(b']') => {
                    return self.token(token_start);
                }
                Some(b'#') => {
                    while let Some(ch) = self.get_byte() {
                        match ch {
                            b'\n' | b'\r' => {
                                self.unget_byte();
                                break;
                            }
                            _ => (),
                        }
                    }
                    return Some(
                        std::str::from_utf8(&self.data[token_start..self.pos]).map_err(Error::from),
                    );
                }
                _ => {
                    // Regular statement or numeric token; scan until we hit a
                    // space, opening quote, or bracket.
                    while let Some(byte) = self.get_byte() {
                        match byte {
                            b' ' | b'\n' | b'\t' | b'\r' | b'"' | b'[' | b']' => {
                                self.unget_byte();
                                break;
                            }
                            _ => (),
                        }
                    }
                    return Some(
                        std::str::from_utf8(&self.data[token_start..self.pos]).map_err(Error::from),
                    );
                }
            }
        }
    }
}

impl<'a> Tokenizer<'a> {
    fn get_byte(&mut self) -> Option<u8> {
        // TODO(wathiede): should we track location information?
        if self.pos == self.data.len() {
            return None;
        }
        let byte = self.data[self.pos];
        self.pos += 1;
        Some(byte)
    }

    fn unget_byte(&mut self) {
        // TODO(wathiede): should we track location information?
        self.pos -= 1;
    }

    fn token(&mut self, token_start: usize) -> Option<Result<&'a str, Error>> {
        Some(std::str::from_utf8(&self.data[token_start..self.pos]).map_err(Error::from))
    }
}

/*
pub fn create_from_file<P: AsRef<Path>>(path: P) -> Tokenizer<'a> {
    Tokenizer {
    }
}
*/

/// Creates a [Tokenizer] from the scene file in `data`.
///
/// [Tokenizer]: crate::core::parser::Tokenizer
pub fn create_from_string<'a>(data: &'a [u8]) -> Tokenizer<'a> {
    Tokenizer { data, pos: 0 }
}

#[derive(PartialEq)]
enum Token {
    Optional,
    Required,
}

#[derive(Default, Debug)]
struct ParamListItem<'a> {
    name: String,
    double_values: Vec<f64>,
    string_values: Vec<&'a str>,
}

impl<'a> ParamListItem<'a> {
    fn size(&self) -> usize {
        self.double_values.len() + self.string_values.len()
    }
}

struct Parser<'a> {
    file_stack: Vec<Tokenizer<'a>>,
    unget_token: Option<&'a str>,
}

impl<'a> Parser<'a> {
    fn parse<A: API>(t: Tokenizer, api: &mut A) -> Result<(), Error> {
        let mut p = Parser {
            file_stack: vec![t],
            unget_token: None,
        };
        // TODO(wathiede): should we track location information?

        loop {
            let tok = p.next_token(Token::Optional);
            let tok = match tok {
                None => break,
                Some(tok) => tok,
            };
            let tok = tok?;
            match tok {
                "Accelerator" => p.basic_param_list_entrypoint(|n, p| api.accelerator(n, p))?,
                "ActiveTransform" => {
                    return Err(Error::NotImplemented("ActiveTransform".to_string()))
                }
                "AreaLightSource" => {
                    return Err(Error::NotImplemented("AreaLightSource".to_string()))
                }
                "AttrbuteBegin" => api.attribute_begin(),
                "AttributeEnd" => api.attribute_end(),
                "Camera" => return Err(Error::NotImplemented("Camera".to_string())),
                "ConcatTransform" => {
                    return Err(Error::NotImplemented("ConcatTransform".to_string()))
                }
                "CoordinateSystem" => {
                    return Err(Error::NotImplemented("CoordinateSystem".to_string()))
                }
                "CoordSysTransform" => {
                    return Err(Error::NotImplemented("CoordSysTransform".to_string()))
                }
                "Film" => p.basic_param_list_entrypoint(|n, p| api.film(n, p))?,
                "Identity" => return Err(Error::NotImplemented("Identity".to_string())),
                "Include" => return Err(Error::NotImplemented("Include".to_string())),
                "Integrator" => return Err(Error::NotImplemented("Integrator".to_string())),
                "LightSource" => return Err(Error::NotImplemented("LightSource".to_string())),
                "LookAt" => {
                    let mut eye: [Float; 3] = Default::default();
                    for i in 0..3 {
                        let tok = p.next_token(Token::Required).unwrap_or(Ok(""))?;
                        eye[i] = tok.parse()?;
                    }

                    let mut look: [Float; 3] = Default::default();
                    for i in 0..3 {
                        let tok = p.next_token(Token::Required).unwrap_or(Ok(""))?;
                        look[i] = tok.parse()?;
                    }

                    let mut up: [Float; 3] = Default::default();
                    for i in 0..3 {
                        let tok = p.next_token(Token::Required).unwrap_or(Ok(""))?;
                        up[i] = tok.parse()?;
                    }
                    api.look_at(eye, look, up);
                }
                "MakeNamedMaterial" => {
                    return Err(Error::NotImplemented("MakeNamedMaterial".to_string()))
                }
                "MakeNamedMedium" => {
                    return Err(Error::NotImplemented("MakeNamedMedium".to_string()))
                }
                "Material" => return Err(Error::NotImplemented("Material".to_string())),
                "MediumInterface" => {
                    return Err(Error::NotImplemented("MediumInterface".to_string()))
                }
                "NamedMaterial" => return Err(Error::NotImplemented("NamedMaterial".to_string())),
                "ObjectBegin" => return Err(Error::NotImplemented("ObjectBegin".to_string())),
                "ObjectEnd" => return Err(Error::NotImplemented("ObjectEnd".to_string())),
                "ObjectInstance" => {
                    return Err(Error::NotImplemented("ObjectInstance".to_string()))
                }
                "PixelFilter" => return Err(Error::NotImplemented("PixelFilter".to_string())),
                "ReverseOrientation" => {
                    return Err(Error::NotImplemented("ReverseOrientation".to_string()))
                }
                "Rotate" => return Err(Error::NotImplemented("Rotate".to_string())),
                "Sampler" => p.basic_param_list_entrypoint(|n, p| api.sampler(n, p))?,
                "Scale" => {
                    let mut v: [Float; 3] = Default::default();
                    for i in 0..3 {
                        let tok = p.next_token(Token::Required).unwrap_or(Ok(""))?;
                        v[i] = tok.parse()?;
                    }
                    api.scale(v[0], v[1], v[2]);
                }
                "Shape" => return Err(Error::NotImplemented("Shape".to_string())),
                "Texture" => return Err(Error::NotImplemented("Texture".to_string())),
                "Transform" => return Err(Error::NotImplemented("Transform".to_string())),
                "TransformBegin" => {
                    return Err(Error::NotImplemented("TransformBegin".to_string()))
                }
                "TransformEnd" => return Err(Error::NotImplemented("TransformEnd".to_string())),
                "TransformTimes" => {
                    return Err(Error::NotImplemented("TransformTimes".to_string()))
                }
                "Translate" => return Err(Error::NotImplemented("Translate".to_string())),
                "WorldBegin" => return Err(Error::NotImplemented("WorldBegin".to_string())),
                "WorldEnd" => return Err(Error::NotImplemented("WorldEnd".to_string())),
                _ => return Err(Error::Syntax(tok.to_string())),
            }
        }
        Ok(())
    }
    // C++ implementation has flags instead of bool, but only two values currently.  Switch to flags
    // if they add more options upstream.
    /// Fetches the next token from the underlying data.  `None` returned at EOF. If data is
    /// available, the inner `Result` will indicate if the token was successfully parsed from the
    /// data.
    fn next_token(&mut self, flags: Token) -> Option<Result<&'a str, Error>> {
        if let Some(token) = self.unget_token.take() {
            return Some(Ok(token));
        }

        // TODO(wathiede): make this file_stack.last() instead of .pop()?  Trying to use .last()
        // fights the borrow checker.
        let tok = match self.file_stack.pop() {
            None => {
                if flags == Token::Required {
                    return Some(Err(Error::EOF));
                }
                return None;
            }
            Some(mut last) => {
                let tok = last.next();
                self.file_stack.push(last);
                tok
            }
        };
        match tok {
            // We've reached EOF in the current file. Anything more to parse?
            None => {
                self.file_stack.pop();
                self.next_token(flags)
            }
            Some(Ok(tok)) if tok.starts_with('#') => self.next_token(flags),
            Some(tok) => Some(tok),
        }
    }

    fn parse_params(&mut self) -> Result<ParamSet, Error> {
        let mut ps = ParamSet::default();
        loop {
            let decl = match self.next_token(Token::Optional) {
                None => return Ok(ps),
                Some(decl) => decl,
            };
            let decl = decl?;

            if !is_quoted_string(decl) {
                self.unget_token = Some(decl);
                return Ok(ps);
            }

            let mut item = ParamListItem {
                name: dequote_string(decl)?.to_string(),
                ..ParamListItem::default()
            };

            // TODO(wathiede): The C++ version uses an arena allocator to manage double_values and
            // string_values.  Profile this at some point and see if the rust version needs a
            // similar optimization.
            let mut add_val = |val| -> Result<(), Error> {
                if is_quoted_string(val) {
                    if !item.double_values.is_empty() {
                        return Err(Error::MixedParameters);
                    }
                    item.string_values.push(val);
                } else {
                    if !item.string_values.is_empty() {
                        return Err(Error::MixedParameters);
                    }
                    item.double_values.push(val.parse::<f64>()?);
                }
                Ok(())
            };

            let val = match self.next_token(Token::Required) {
                None => return Ok(ps),
                Some(val) => val,
            };
            let val = val?;
            if val == "[" {
                loop {
                    let val = match self.next_token(Token::Required) {
                        None => return Ok(ps),
                        Some(val) => val,
                    };
                    let val = val?;
                    if val == "]" {
                        break;
                    }
                    add_val(val)?;
                }
            } else {
                add_val(val)?;
            }
            add_param(&mut ps, item);
        }
    }

    fn basic_param_list_entrypoint<F: FnMut(&str, ParamSet)>(
        &mut self,
        mut api_func: F,
    ) -> Result<(), Error> {
        let token = match self.next_token(Token::Required) {
            None => return Err(Error::Unquoted("".to_string())),
            Some(token) => token,
        };
        let token = token?;
        let n = dequote_string(token)?;
        let params = self.parse_params()?;
        dbg!(&params);
        api_func(n, params);
        Ok(())
    }
}

#[derive(Debug)]
enum ParamType {
    Int,
    Bool,
    Float,
    Point2,
    Vector2,
    Point3,
    Vector3,
    Normal,
    RGB,
    XYZ,
    Blackbody,
    Spectrum,
    String,
    Texture,
}

impl TryFrom<&str> for ParamType {
    type Error = String;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let p_type = match input {
            "integer" => ParamType::Int,
            "bool" => ParamType::Bool,
            "float" => ParamType::Float,
            "point2" => ParamType::Point2,
            "vector2" => ParamType::Vector2,
            "point3" => ParamType::Point3,
            "vector3" => ParamType::Vector3,
            "normal" => ParamType::Normal,
            "color" => ParamType::RGB,
            "rgb" => ParamType::RGB,
            "xyz" => ParamType::XYZ,
            "blackbody" => ParamType::Blackbody,
            "spectrum" => ParamType::Spectrum,
            "string" => ParamType::String,
            "texture" => ParamType::Texture,
            _ => return Err(format!("unknown parameter type '{}'", input)),
        };
        Ok(p_type)
    }
}

fn lookup_type(decl: &str) -> Option<(ParamType, &str)> {
    let p_type = decl.trim_start();
    if p_type.is_empty() {
        error!("Parameter '{}' doesn't have a type declaration?!", decl);
        return None;
    }
    let (p_type, p_name) = match p_type.find(&[' ', '\t'][..]) {
        Some(idx) => (&p_type[..idx], p_type[idx..].trim()),
        None => {
            error!("Parameter '{}' missing space before name", decl);
            return None;
        }
    };
    let p_type = match ParamType::try_from(p_type) {
        Ok(p_type) => p_type,
        Err(e) => {
            error!("Unable to decode type from '{}': {}", decl, e);
            return None;
        }
    };
    if p_name.is_empty() {
        error!("Unable to find parameter name from '{}'", decl);
        return None;
    }
    Some((p_type, p_name))
}

fn add_param(ps: &mut ParamSet, item: ParamListItem) {
    fn iter2d<'a>(items: &'a [f64]) -> impl Iterator<Item = (Float, Float)> + 'a {
        let xs =
            items
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| if i % 2 == 0 { Some(v as Float) } else { None });
        let ys =
            items
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| if i % 2 == 1 { Some(v as Float) } else { None });
        xs.zip(ys)
    };
    fn iter3d<'a>(items: &'a [f64]) -> impl Iterator<Item = (Float, Float, Float)> + 'a {
        let xs =
            items
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| if i % 3 == 0 { Some(v as Float) } else { None });
        let ys =
            items
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| if i % 3 == 1 { Some(v as Float) } else { None });
        let zs =
            items
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| if i % 3 == 2 { Some(v as Float) } else { None });
        xs.zip(ys).zip(zs).map(|((x, y), z)| (x, y, z))
    };
    match lookup_type(&item.name) {
        Some((p_type, p_name)) => {
            match p_type {
                ParamType::Texture | ParamType::String | ParamType::Bool => {
                    if item.string_values.is_empty() {
                        error!(
                        "Expected string parameter value for parameter '{}' with type '{:?}' Ignoring.",
                        p_name, p_type
                    );
                        return;
                    }
                }
                ParamType::Int
                | ParamType::Float
                | ParamType::Point2
                | ParamType::Vector2
                | ParamType::Point3
                | ParamType::Vector3
                | ParamType::Normal
                | ParamType::RGB
                | ParamType::XYZ
                | ParamType::Blackbody => {
                    if !item.string_values.is_empty() {
                        error!(
                        "Expected numeric parameter value for parameter '{}' with type '{:?}' Ignoring.",
                        p_name, p_type
                    );
                        return;
                    }
                }

                // Spectrum can be strings or numeric.
                ParamType::Spectrum => (),
            };
            let n_items = item.size();

            match p_type {
                ParamType::Int => {
                    ps.add_int(
                        p_name,
                        item.double_values.iter().map(|f| *f as isize).collect(),
                    );
                }
                ParamType::Bool => ps.add_bool(
                    p_name,
                    item.string_values
                        .iter()
                        // TODO |&s| and drop the *s
                        .map(|s| match *s {
                            "true" => true,
                            "false" => false,
                            _ => {
                                warn!(
                                    "Value '{}' unknown for Boolean parameter '{}'. Using 'false'.",
                                    s, item.name
                                );
                                false
                            }
                        })
                        .collect(),
                ),
                ParamType::Float => {
                    ps.add_float(
                        p_name,
                        item.double_values.iter().map(|f| *f as Float).collect(),
                    );
                }
                ParamType::Point2 => {
                    if (n_items % 2) != 0 {
                        warn!("Excess values given with point2 parameter '{}'. Ignoring last one of them.", item.name);
                    }
                    ps.add_point2f(
                        p_name,
                        iter2d(&item.double_values)
                            .map(|xy| Point2f::from(xy))
                            .collect(),
                    );
                }
                ParamType::Vector2 => {
                    if (n_items % 2) != 0 {
                        warn!("Excess values given with vector2 parameter '{}'. Ignoring last one of them.", item.name);
                    }
                    ps.add_vector2f(
                        p_name,
                        iter2d(&item.double_values)
                            .map(|xy| Vector2f::from(xy))
                            .collect(),
                    );
                }
                ParamType::Point3 => {
                    if (n_items % 3) != 0 {
                        warn!("Excess values given with point3 parameter '{}'. Ignoring last {} of them.", item.name, n_items%3);
                    }
                    ps.add_point3f(
                        p_name,
                        iter3d(&item.double_values)
                            .map(|xyz| Point3f::from(xyz))
                            .collect(),
                    );
                }
                ParamType::Vector3 => {
                    if (n_items % 3) != 0 {
                        warn!("Excess values given with vector3 parameter '{}'. Ignoring last {} of them.", item.name, n_items%3);
                    }
                    ps.add_vector3f(
                        p_name,
                        iter3d(&item.double_values)
                            .map(|xyz| Vector3f::from(xyz))
                            .collect(),
                    );
                }
                ParamType::Normal => {
                    if (n_items % 3) != 0 {
                        warn!("Excess values given with normal parameter '{}'. Ignoring last {} of them.", item.name, n_items%3);
                    }
                    ps.add_normal3f(
                        p_name,
                        iter3d(&item.double_values)
                            .map(|xyz| Normal3f::from(xyz))
                            .collect(),
                    );
                }
                ParamType::RGB => {
                    if (n_items % 3) != 0 {
                        warn!("Excess RGB values given with parameter '{}'. Ignoring last {} of them.", item.name, n_items%3);
                    }
                    let end = n_items - n_items % 3;
                    ps.add_rgb_spectrum(
                        p_name,
                        item.double_values
                            .iter()
                            .take(end)
                            .map(|&f| f as Float)
                            .collect(),
                    );
                }
                ParamType::XYZ => {
                    if (n_items % 3) != 0 {
                        warn!("Excess XYZ values given with parameter '{}'. Ignoring last {} of them.", item.name, n_items%3);
                    }
                    let end = n_items - n_items % 3;
                    ps.add_rgb_spectrum(
                        p_name,
                        item.double_values
                            .iter()
                            .take(end)
                            .map(|&f| f as Float)
                            .collect(),
                    );
                }
                ParamType::Blackbody => {
                    if (n_items % 2) != 0 {
                        warn!(
                            "Excess value given with blackbody parameter '{}'. Ignoring extra one.",
                            item.name
                        );
                    }
                    let end = n_items - n_items % 2;
                    ps.add_blackbody(
                        p_name,
                        item.double_values
                            .iter()
                            .take(end)
                            .map(|&f| f as Float)
                            .collect(),
                    );
                }
                ParamType::Spectrum => {
                    if !item.string_values.is_empty() {
                        ps.add_sampled_spectrum_files(
                            p_name,
                            item.string_values.iter().map(|s| s.to_string()).collect(),
                        );
                    } else {
                        if (n_items % 2) != 0 {
                            warn!(
                            "Non-even number of values given with sampled spectrum '{}'. Ignoring extra.",
                            item.name
                        );
                        }
                        let end = n_items - n_items % 2;
                        ps.add_sampled_spectrum(
                            p_name,
                            item.double_values
                                .iter()
                                .take(end)
                                .map(|&f| f as Float)
                                .collect(),
                        );
                    }
                }
                ParamType::String => {
                    ps.add_string(
                        p_name,
                        item.string_values.iter().map(|s| s.to_string()).collect(),
                    );
                }
                ParamType::Texture => {
                    if n_items == 1 {
                        ps.add_texture(p_name, item.string_values[0].to_string());
                    } else {
                        error!(
                            "Only one string allowed for 'texture' paramter '{}'",
                            p_name
                        );
                    }
                }
            }
        }
        None => warn!("Type of parameter '{}' is unknown", item.name),
    }
}

fn is_quoted_string(s: &str) -> bool {
    s.len() >= 2 && s.starts_with("\"") && s.ends_with("\"")
}

fn dequote_string(s: &str) -> Result<&str, Error> {
    if !is_quoted_string(s) {
        return Err(Error::Unquoted(s.to_string()));
    }
    Ok(&s[1..s.len() - 1])
}

/// Parse the tokens provided by `t` and called the appropriate methos on `a`.
pub fn parse<A: API>(t: Tokenizer, api: &mut A) -> Result<(), Error> {
    Parser::parse(t, api)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::api_test::MockAPI;

    #[test]
    fn tokenizer() {
        let mut t = create_from_string(r#"Sampler "halton" "integer pixelsamples" 128"#.as_bytes());
        assert_eq!(Some(Ok("Sampler")), t.next());
        assert_eq!(Some(Ok(r#""halton""#)), t.next());
        assert_eq!(Some(Ok(r#""integer pixelsamples""#)), t.next());
        assert_eq!(Some(Ok("128")), t.next());
        assert_eq!(None, t.next());

        let mut t = create_from_string(r#"Sampler "128"#.as_bytes());
        assert_eq!(Some(Ok("Sampler")), t.next());
        assert_eq!(Some(Err(Error::EOF)), t.next());
    }

    #[test]
    fn parser() {
        let mut api = MockAPI::default();
        let t = create_from_string(r#"Sampler "halton" "integer pixelsamples" 128"#.as_bytes());
        let res = parse(t, &mut api);
        assert!(res.is_ok(), "error from parse: {}", res.err().unwrap());
    }
}
