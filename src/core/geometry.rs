use core::pbrt::{Float, Int};

#[derive(Debug, Clone, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

pub type Vector2f = Vector2<Float>;
pub type Vector2i = Vector2<Int>;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vector3f = Vector3<Float>;
pub type Vector3i = Vector3<Int>;

#[derive(Debug, Clone, PartialEq)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

pub type Point2f = Point2<Float>;
pub type Point2i = Point2<Int>;

#[derive(Debug, Clone, PartialEq)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Point3f = Point3<Float>;
pub type Point3i = Point3<Int>;

#[derive(Debug, Clone, PartialEq)]
pub struct Normal3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Normal3f = Normal3<Float>;
