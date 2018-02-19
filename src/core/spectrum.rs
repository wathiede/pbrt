use core::pbrt::Float;

// TODO(wathiede): this is really wrong, but it's a placeholder so code should compile.  See
// chapter 5 section 1-3.
#[derive(Debug, Clone, PartialEq)]
pub struct Spectrum {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
