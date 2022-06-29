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

//! The main parts of the PBRT renderer are located in sub crates of `core`.  This top-level crate
//! has no public functionality.

pub mod api;
// Public so pbrt-compare can use it.
pub mod api_test;
pub mod error;
pub mod film;
pub mod filter;
pub mod geometry;
pub mod imageio;
pub mod interaction;
pub mod light;
pub mod medium;
pub mod mipmap;
pub mod parallel;
pub mod paramset;
pub mod parser;
pub mod rng;
pub mod sampling;
pub mod sobolmatrices;
pub mod spectrum;
pub mod texture;
pub mod transform;
