extern crate pbrt;

use pbrt::core::pbrt::Float;
use pbrt::core::paramset::TextureParams;
use pbrt::textures::constant::ConstantTexture;

#[test]
fn test_texture_params() {
    let mut tp = TextureParams::new();
    tp.float_textures.insert(
        "constant".to_owned(),
        Box::new(ConstantTexture::new(0.25 as Float)),
    );
}
