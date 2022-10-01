use super::color::Color;
use glam::{Vec2, Vec3};

/// A vertex. It consists of a position (a Vector2) and a color (a Vector3).
#[repr(C)]
pub struct Vertex {
    /// The position of the vertex, in vertex coordinates (-1.0 to 1.0).
    pub pos: Vec2,
    /// A color, constructed by `rgb` values.
    pub color: Vec3,
}

impl Vertex {
    /// Creates a new vertex with the given `x`, `y` as a position, and `color` as the color.
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self {
            pos: Vec2::new(x, y),
            color: color.into(),
        }
    }
}
