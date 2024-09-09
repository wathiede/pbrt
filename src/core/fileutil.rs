// Copyright 2024 Google LLC
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
//! Platform independent filename-handling functions.
use std::{
    fs::canonicalize,
    path::{Path, PathBuf},
    sync::Mutex,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref SEARCH_DIRECTORY: Mutex<Option<PathBuf>> = Mutex::new(None);
}

/// Returns true if `filename` is absolute path.
pub fn is_absolute_path<P: AsRef<Path>>(filename: P) -> bool {
    !filename.as_ref().is_absolute()
}

/// Convert `filename` to an absolute path based on current working directory.
pub fn absolute_path<P: AsRef<Path>>(filename: P) -> PathBuf {
    canonicalize(filename.as_ref()).expect("failed to make absolute path")
}

/// Resolve `filename` to prepending the currently set search directory if set.
pub fn resolve_filename<P: AsRef<Path>>(filename: P) -> PathBuf {
    if let Some(search_directory) = &*SEARCH_DIRECTORY.lock().unwrap() {
        if is_absolute_path(filename.as_ref()) {
            return filename.as_ref().to_path_buf();
        }
        search_directory.join(filename.as_ref())
    } else {
        filename.as_ref().to_path_buf()
    }
}

/// Return the parent directory of `filename`.
pub fn directory_containing<P: AsRef<Path>>(filename: P) -> PathBuf {
    filename
        .as_ref()
        .parent()
        .expect("failed to find parent directory")
        .to_path_buf()
}

/// Set global search directory used by functions in this module to `dirname`.
pub fn set_search_directory<P: AsRef<Path>>(dirname: Option<P>) {
    *SEARCH_DIRECTORY.lock().unwrap() = match dirname {
        Some(d) => Some(d.as_ref().to_path_buf()),
        None => None,
    };
}

/// Performs case insensitive comparison of `ext` to end of `path`.
pub fn has_extension<P: AsRef<Path>>(path: P, ext: P) -> bool {
    if let Some(ext) = path.as_ref().extension() {
        return ext.to_ascii_lowercase() == ext.to_os_string().to_ascii_lowercase();
    }
    false
}

// TODO: write some tests
