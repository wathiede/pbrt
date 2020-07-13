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

/// Error type for tokenization and parsing errors.
#[derive(PartialEq, Debug, Error)]
pub enum Error {
    /// Inpute data isn't valid utf-8.
    #[error("input not utf-8")]
    StrError(#[from] std::str::Utf8Error),
    /// Quoted string without closing quote.
    #[error("unterminated string")]
    UnterminatedString,
    /// Hit end-of-file unexpectedly while parsing.
    #[error("premature EOF")]
    EOF,
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
