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
use core::interaction::SurfaceInteraction;
use core::texture::Texture;

pub struct ConstantTexture<T> {
    value: T,
}

impl<T> ConstantTexture<T> {
    pub fn new(value: T) -> ConstantTexture<T> {
        ConstantTexture { value }
    }
}

impl<T> Texture for ConstantTexture<T>
where
    T: Clone,
{
    type Output = T;

    fn evaluate(&self, _si: &SurfaceInteraction) -> Self::Output {
        self.value.clone()
    }
}
