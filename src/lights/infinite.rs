// Copyright 2020 Google LLC
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

//! [Light] implementation for infinite area light.
//!
//! [Light]: crate::core::light::Light
use std::sync::Arc;

use crate::{
    core::{
        geometry::Point3f,
        imageio::read_image,
        light::{Light, LightData},
        mipmap::MIPMap,
        paramset::ParamSet,
        sampling::Distribution2D,
        spectrum::{RGBSpectrum, Spectrum},
        transform::Transform,
    },
    Float,
};

#[derive(Debug)]
/// InfiniteAreaLight represents a light infinitely far away that surrounds the entire scene.
pub struct InfiniteAreaLight {
    light_data: LightData,
    lmap: MIPMap<RGBSpectrum>,
    world_center: Point3f,
    world_radius: Float,
    distribution: Distribution2D,
}

impl Light for InfiniteAreaLight {}
impl InfiniteAreaLight {
    fn new(
        _light2world: &Transform,
        l: &Spectrum,
        _n_samples: isize,
        texmap: &str,
    ) -> InfiniteAreaLight {
        let (texels, resolution) = if !texmap.is_empty() {
            if let Ok((mut texels, resolution)) = read_image(texmap) {
                texels.iter_mut().for_each(|p| *p *= l.to_rgb_spectrum());
                (texels, resolution)
            } else {
                (vec![l.to_rgb_spectrum()], [1, 1].into())
            }
        } else {
            (vec![l.to_rgb_spectrum()], [1, 1].into())
        };
        let _ = texels;
        let _ = resolution;
        //lmap.reset(MIPMap::new(resolution, texels));

        todo!("InfiniteAreaLight::new()");
        /*
        InfiniteAreaLight {
            light_data: LightData::new(LightFlags::Infinite, n_samples, MediumInterface::default()),
            lmap,
            world_center,
            world_radius,
            distribution,
        }
        */
    }
}

/// Creates an InfiniteAreaLight with the given `Transform` and parameters.
pub fn create_infinite_light(light2world: &Transform, params: &ParamSet) -> Arc<InfiniteAreaLight> {
    let l = params.find_one_spectrum("L", Spectrum::new(1.0));
    let sc = params.find_one_spectrum("scale", Spectrum::new(1.0));
    let texmap = params.find_one_filename("mapname", "");
    let n_samples = params.find_one_int("samples", params.find_one_int("nsamples", 1));
    // TODO(wathiede): do we plumb options into this constructor or make options a singleton random
    // things can grab?
    //if (PbrtOptions.quickRender) nSamples =  (n_samples / 4).max(1);
    Arc::new(InfiniteAreaLight::new(
        light2world,
        &(l * sc),
        n_samples,
        &texmap,
    ))
}
