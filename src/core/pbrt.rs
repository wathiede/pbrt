// Set this type alias to modify all floats in pbrt to be 32 or 64-bit.
pub type Float = f32;

#[derive(Clone, Debug, Default)]
pub struct Options {
    pub num_threads: u32,
    pub quick_render: bool,
    pub quiet: bool,
    pub verbose: bool,
    pub image_file: String,
}
