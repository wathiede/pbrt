use structopt::StructOpt;

// Set this type alias to modify all floats in pbrt to be 32 or 64-bit.
pub type Float = f32;
// Set this type alias to modify all ints in pbrt to be 32 or 64-bit.
pub type Int = i32;

#[derive(Clone, Debug, Default, StructOpt)]
#[structopt(name = "pbrt", about = "Rust implementation of http://pbrt.org/")]
pub struct Options {
    #[structopt(short = "n", long = "nthreads")]
    /// Use specified number of threads for rendering.
    pub num_threads: Option<u32>,
    #[structopt(long = "quick")]
    /// Automatically reduce a number of quality settings to render more quickly.
    pub quick_render: bool,
    #[structopt(short = "q", long = "quiet")]
    /// Suppress all text output other than error messages.
    pub quiet: bool,
    #[structopt(short = "v", long = "verbose")]
    /// Print out more detailed logging information.
    pub verbose: bool,
    #[structopt(short = "o", long = "outfile")]
    /// Write the final image to the given filename.
    pub image_file: Option<String>,
    pub scene_files: Vec<String>,
}

const PI: Float = 3.14159265358979323846;
const INV_PI: Float = 0.31830988618379067154;
const INV2_PI: Float = 0.15915494309189533577;
const INV4_PI: Float = 0.07957747154594766788;
const PI_OVER2: Float = 1.57079632679489661923;
const PI_OVER4: Float = 0.78539816339744830961;
const SQRT2: Float = 1.41421356237309504880;

pub fn radians(deg: Float) -> Float {
    (PI / 180.) * deg
}

pub fn degrees(rad: Float) -> Float {
    (180. / PI) * rad
}
