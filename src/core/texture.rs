use core::pbrt::Float;
use core::interaction::SurfaceInteraction;
use core::spectrum::Spectrum;

pub trait Texture {
    type Output;

    fn evaluate(&self, _si: &SurfaceInteraction) -> Self::Output;
}
