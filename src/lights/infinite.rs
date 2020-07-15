use std::sync::Arc;

use crate::core::light::Light;
use crate::core::paramset::ParamSet;
use crate::core::transform::Transform;

#[derive(Debug)]
pub struct InfiniteAreaLight {
    /*
// InfiniteAreaLight Private Data
std::unique_ptr<MIPMap<RGBSpectrum>> Lmap;
Point3f worldCenter;
Float worldRadius;
std::unique_ptr<Distribution2D> distribution;
 */}

impl Light for InfiniteAreaLight {}

pub fn create_infinite_light(light2world: &Transform, params: &ParamSet) -> Arc<InfiniteAreaLight> {
    todo!("lights::infinite::create_infinite_light");
}
