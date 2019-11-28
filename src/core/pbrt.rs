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

#[derive(Copy, Clone)]
pub struct Degree(pub(crate) Float);

impl From<Float> for Degree {
    fn from(f: Float) -> Degree {
        Degree(f)
    }
}

#[derive(Clone, Debug)]
pub struct Options {
    pub num_threads: u32,
    pub quick_render: bool,
    pub quiet: bool,
    pub verbose: bool,
    pub image_file: String,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            num_threads: 1,
            quick_render: false,
            quiet: false,
            verbose: true,
            image_file: "".to_owned(),
        }
    }
}

//const PI: Float = 3.14159265358979323846;
//const INV_PI: Float = 0.31830988618379067154;
//const INV2_PI: Float = 0.15915494309189533577;
//const INV4_PI: Float = 0.07957747154594766788;
//const PI_OVER2: Float = 1.57079632679489661923;
//const PI_OVER4: Float = 0.78539816339744830961;
//const SQRT2: Float = 1.41421356237309504880;

/// Linear interpolate `t` between `v1` and `v2`.
///
/// # Examples
/// ```
/// # use pbrt::core::pbrt::lerp;
/// assert_eq!(lerp(0., 0., 1.), 0.);
/// assert_eq!(lerp(0.5, 0., 1.), 0.5);
/// assert_eq!(lerp(1., 0., 1.), 1.);
/// assert_eq!(lerp(0.75, 0., 2.), 1.5);
/// ```
pub fn lerp(t: Float, v1: Float, v2: Float) -> Float {
    (1. - t) * v1 + t * v2
}

/// Find roots of quadratic equation, if they exist.
///
/// # Examples
/// From
/// https://www.cliffsnotes.com/study-guides/algebra/algebra-i/quadratic-equations/solving-quadratic-equations
/// ```
/// # use pbrt::core::pbrt::quadratic;
///
/// assert_eq!(quadratic(1., -6., -16.), Some((-2., 8.)));
/// assert_eq!(quadratic(1., 6., 5.), Some((-5., -1.)));
/// assert_eq!(quadratic(1., 0., -16.), Some((-4. ,4.)));
/// assert_eq!(quadratic(1., 6., 0.), Some((-6. ,0.)));
/// // Extra precision nescessary to match the output of quadratic which computes its answer with
/// // higher precision.
/// assert_eq!(quadratic(1., 2., -2.), Some(((-1.-3_f64.sqrt()) as f32, (-1.+3_f64.sqrt()) as f32)));
pub fn quadratic(a: Float, b: Float, c: Float) -> Option<(Float, Float)> {
    let a = a as f64;
    let b = b as f64;
    let c = c as f64;
    // Find quadratic discriminant
    let discrim = b * b - 4. * a * c;
    if discrim < 0. {
        return None;
    }
    let root_discrim = discrim.sqrt();
    let q = if b < 0. {
        -0.5 * (b - root_discrim)
    } else {
        -0.5 * (b + root_discrim)
    };
    let t0 = (q / a) as Float;
    let t1 = (c / q) as Float;
    if t0 > t1 {
        Some((t1, t0))
    } else {
        Some((t0, t1))
    }
}
