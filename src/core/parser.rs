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

use thiserror::Error;

use crate::core::api::API;
use crate::core::paramset::ParamSet;
use crate::core::spectrum::SpectrumType;

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

struct Parser<'a> {
    file_stack: Vec<Tokenizer<'a>>,
    unget_token: Option<&'a str>,
}

impl<'a> Parser<'a> {
    fn parse<A: API>(t: Tokenizer, mut api: A) -> Result<(), Error> {
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
                "AttrbuteBegin" => api.attribute_begin(),
                "Sampler" => p.basic_param_list_entrypoint(SpectrumType::Reflectance, |n, p| {
                    api.sampler(n, p)
                })?,
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

    fn parse_params(&mut self, spectrum_type: SpectrumType) -> Result<ParamSet, Error> {
        let ps = ParamSet::default();
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

            add_param(&ps, item, &spectrum_type);
        }
    }

    fn basic_param_list_entrypoint<F: FnMut(&str, ParamSet)>(
        &mut self,
        spectrum_type: SpectrumType,
        mut api_func: F,
    ) -> Result<(), Error> {
        let token = match self.next_token(Token::Required) {
            None => return Err(Error::Unquoted("".to_string())),
            Some(token) => token,
        };
        let token = token?;
        let n = dequote_string(token)?;
        let params = self.parse_params(spectrum_type)?;
        api_func(n, params);
        Ok(())
    }
}

fn add_param(ps: &ParamSet, item: ParamListItem, spectrum_type: &SpectrumType) {
    dbg!(ps, item, spectrum_type);
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
pub fn parse<A: API>(t: Tokenizer, api: A) -> Result<(), Error> {
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
        let api = MockAPI::default();
        let t = create_from_string(r#"Sampler "halton" "integer pixelsamples" 128"#.as_bytes());
        let res = parse(t, api);
        assert!(res.is_ok(), "error from parse: {}", res.err().unwrap());
    }
}
