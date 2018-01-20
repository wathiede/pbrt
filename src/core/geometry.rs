use core::pbrt::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector2f {
    pub x: Float,
    pub y: Float,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point2f {
    pub x: Float,
    pub y: Float,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Normal3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
