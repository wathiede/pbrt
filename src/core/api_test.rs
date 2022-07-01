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

//! Implementation of [API] for testing.
use std::path::Path;

use crate::{
    core::{
        api::{Error, API},
        paramset::ParamSet,
        parser::Error as ParserError,
    },
    Degree, Float,
};

/// MockAPI is an empty implementation of [API] used for testing the parser.
#[derive(Default)]
pub struct MockAPI {}
impl API for MockAPI {
    /// Sets the renderer's accelerator settings to `name` & `params`.
    fn accelerator(&mut self, _name: &str, _params: ParamSet) {
        // unimplemented!()
    }
    /// Sets the active transform bits to `ALL_TRANSFORMS_BITS`.
    fn active_transform_all(&mut self) {
        // unimplemented!()
    }
    /// Sets the active transform bits to `END_TRANSFORMS_BITS`.
    fn active_transform_end_time(&mut self) {
        // unimplemented!()
    }
    /// Sets the active transform bits to `START_TRANSFORMS_BITS`.
    fn active_transform_start_time(&mut self) {
        // unimplemented!()
    }
    /// Creates area light when `AreaLightSource` found in scene.
    fn area_light_source(&mut self, _name: &str, _params: ParamSet) {
        // unimplemented!()
    }
    /// Called when parser sees a `AttributeBegin` keyword
    fn attribute_begin(&mut self) {
        // unimplemented!()
    }
    /// Called when parser sees a `AttributeEnd` keyword
    fn attribute_end(&mut self) {
        // unimplemented!()
    }
    /// Sets the renderer's camera settings to `name` & `params`.
    fn camera(&mut self, _name: &str, _params: ParamSet) {
        // unimplemented!()
    }
    /// Reset the internal state of self.
    fn cleanup(&mut self) {
        // unimplemented!()
    }
    /// Multiples the current transform matrix by `transform`.
    fn concat_transform(&mut self, _transform: [Float; 16]) {
        // unimplemented!()
    }
    /// Creates a new coordinate system assigning `name` the current transform matrix.
    fn coordinate_system(&mut self, _name: &str) {
        // unimplemented!()
    }
    /// Sets the current transform matrix to the one stored under `name`.
    fn coordinate_system_transform(&mut self, _name: &str) {
        // unimplemented!()
    }
    /// Sets the renderer's film settings to `name` & `params`.
    fn film(&mut self, _name: &str, _params: ParamSet) {
        // unimplemented!()
    }
    /// Sets the currently active transform matrix by the given values.
    fn identity(&mut self) {
        // unimplemented!()
    }
    /// Moves the internal statemachine from `APIState::Uninitialized` to `APIState::OptionsBlock`.
    /// This function must be called before most of the API will work.
    fn init(&mut self) {
        // unimplemented!()
    }
    /// Sets the renderer's integrator settings to `name` & `params`.
    fn integrator(&mut self, _name: &str, _params: ParamSet) {
        // unimplemented!()
    }
    /// Creates light when `LightSource` found in scene.
    fn light_source(&mut self, _name: &str, _params: ParamSet) {
        // unimplemented!()
    }
    /// Sets the current transforms to look at the given directions.
    fn look_at(&mut self, _eye: [Float; 3], _look: [Float; 3], _up: [Float; 3]) {
        // unimplemented!()
    }
    /// Creates a medium with the given `params` and stores it as a named media under `name`.
    fn make_named_medium(&mut self, _name: &str, _params: &mut ParamSet) {
        // unimplemented!()
    }
    /// Specifies the current inside and outside media by the names given.  Cameras and lights
    /// without geometry ignore the `inside_name`.
    fn medium_interface(&mut self, _inside_name: &str, _outside_name: &str) {
        // unimplemented!()
    }
    /// Parse a scene file at `path` on the file-system.  This will parse the contents of the file
    /// generating an inmemory representation of the scene, and trigger the rendering and output of
    /// the image.
    fn parse_file<P: AsRef<Path>>(&mut self, _path: P) -> Result<(), Error> {
        Err(ParserError::EOF.into())
    }
    /// Parse a scene file represented as text stored in `data`.  This will parse the contents of
    /// data generating an inmemory representation of the scene, and trigger the rendering and
    /// output of
    /// the image.
    fn parse_string(&mut self, _data: &[u8]) -> Result<(), Error> {
        Err(ParserError::EOF.into())
    }
    /// Sets the renderer's filter settings to `name` & `params`.
    fn pixel_filter(&mut self, _name: &str, _params: ParamSet) {
        // unimplemented!()
    }
    /// Rotates the currently active transform matrix by the given values.
    fn rotate(&mut self, _angle: Degree, _ax: Float, _ay: Float, _az: Float) {
        // unimplemented!()
    }
    /// Sets the renderer's sampler settings to `name` & `params`.
    fn sampler(&mut self, _name: &str, _params: ParamSet) {
        // unimplemented!()
    }
    /// Scales the currently active transform matrix by the given values.
    fn scale(&mut self, _sx: Float, _sy: Float, _sz: Float) {
        // unimplemented!()
    }
    /// Called when the parser sees a `Texture` line.
    fn texture(&mut self, _name: &str, _kind: &str, _texname: &str, _params: ParamSet) {
        // unimplemented!()
    }
    /// Called when parser sees a `TransformBegin` keyword
    fn transform_begin(&mut self) {
        // unimplemented!()
    }
    /// Called when parser sees a `TransformEnd` keyword
    fn transform_end(&mut self) {
        // unimplemented!()
    }
    /// Sets the current transform matrix to `transform`.
    fn transform(&mut self, _transform: [Float; 16]) {
        // unimplemented!()
    }
    /// Sets the start/end times for the transform matrix to `start` & `end`.
    fn transform_times(&mut self, _start: Float, _end: Float) {
        // unimplemented!()
    }
    /// Translates the currently active transform matrix by the given values.
    fn translate(&mut self, _dx: Float, _dy: Float, _dz: Float) {
        // unimplemented!()
    }
    /// Called when parser sees a `WorldBegin` keyword
    fn world_begin(&mut self) {
        // unimplemented!()
    }
    /// Called when parser sees a `WorldEnd` keyword
    fn world_end(&mut self) {
        // unimplemented!()
    }
}
