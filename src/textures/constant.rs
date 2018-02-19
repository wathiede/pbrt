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
