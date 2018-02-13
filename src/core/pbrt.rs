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
// Set this type alias to modify all floats in pbrt to be 32 or 64-bit.
use std::f32;
pub type Float = f32;
pub const EPSILON: Float = f32::EPSILON;
// Set this type alias to modify all ints in pbrt to be 32 or 64-bit.
pub type Int = i32;

#[derive(Clone, Debug, Default)]
pub struct Options {
    pub num_threads: u32,
    pub quick_render: bool,
    pub quiet: bool,
    pub verbose: bool,
    pub image_file: String,
}

//const PI: Float = 3.14159265358979323846;
//const INV_PI: Float = 0.31830988618379067154;
//const INV2_PI: Float = 0.15915494309189533577;
//const INV4_PI: Float = 0.07957747154594766788;
//const PI_OVER2: Float = 1.57079632679489661923;
//const PI_OVER4: Float = 0.78539816339744830961;
//const SQRT2: Float = 1.41421356237309504880;
