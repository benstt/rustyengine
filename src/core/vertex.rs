use super::color::Color;
use glam::{Vec2, Vec3};

#[repr(C)]
pub struct Vertex {
    pub pos: Vec2,
    pub color: Vec3,
}

impl Vertex {
    /// Creates a new vertex with the given x, y as a position.
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self {
            pos: Vec2::new(x, y),
            color: color.into(),
        }
    }
}
