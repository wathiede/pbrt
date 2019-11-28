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
use std::fmt::Debug;

use crate::core::interaction::SurfaceInteraction;

pub trait Texture<T>: Debug
where
    T: Debug,
{
    fn evaluate(&self, _si: &SurfaceInteraction) -> T;
}

impl<T> Texture<T> for Box<dyn Texture<T>>
where
    T: Debug,
{
    fn evaluate(&self, si: &SurfaceInteraction) -> T {
        (**self).evaluate(si)
    }
}
