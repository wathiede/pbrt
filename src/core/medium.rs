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

//! Mediums represent volumetric scattering.

use std::{fmt::Debug, sync::Arc};

// TODO(wathiede): This is a virtual base class in C++, can we make it a trait?  How do you have a
// collection of trait objects?
/// Stub type for flushing out [PbrtAPI].  TODO(wathiede): actually implement and document.
///
/// [PbrtAPI]: crate::core::api::PbrtAPI
pub trait Medium: Debug {}

#[derive(Debug, Default)]
/// MediumInterface defines the border between two media.
pub struct MediumInterface {
    /// The `Medium` inside the object.
    pub inside: Option<Arc<dyn Medium>>,
    /// The `Medium` outside the object.
    pub outside: Option<Arc<dyn Medium>>,
}
