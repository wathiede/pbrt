use crate::core::geometry::Point2f;
use crate::core::geometry::Vector2f;
use crate::Float;

pub trait Filter {
    /// evalute the filter at the given point `p`.
    fn evaluate(&self, p: Point2f) -> Float;
    /// return the radius this filter was created with.
    fn radius(&self) -> Vector2f;
    /// return the inv_radius this filter was created with.
    fn inv_radius(&self) -> Vector2f;
}
