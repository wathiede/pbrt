// Set this type alias to modify all floats in pbrt to be 32 or 64-bit.
pub type Float = f32;
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
