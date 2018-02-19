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
use core::pbrt::Float;

// TODO(wathiede): this is really wrong, but it's a placeholder so code should compile.  See
// chapter 5 section 1-3.
#[derive(Debug, Clone, PartialEq)]
pub struct Spectrum {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
