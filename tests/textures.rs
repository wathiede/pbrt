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
extern crate pbrt;

use std::collections::HashMap;
use std::sync::Arc;

use pbrt::core::api;
use pbrt::core::paramset::ParamSet;
use pbrt::core::paramset::TextureParams;
use pbrt::core::pbrt::Float;
use pbrt::core::pbrt::Options;
use pbrt::core::texture::Texture;
use pbrt::core::transform::Transform;
use pbrt::textures::constant::ConstantTexture;

#[test]
fn test_constant_float_texture_default() {
    let p = Default::default();
    let opts = Default::default();
    let ref mut pbrt = api::Pbrt::new(&opts);
    pbrt.init();
    pbrt.world_begin();
    pbrt.texture("tex1", "float", "constant", p);
    // TODO(wathiede): assert things against pbrt.graphics_state.float_textures
}
