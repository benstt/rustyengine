use super::color::Color;
use glam::{Vec2, Vec3};

/// A vertex. It consists of a position (a Vector2) and a color (a Vector3).
#[repr(C)]
pub struct Vertex {
    /// The position of the vertex, in vertex coordinates (-1.0 to 1.0).
    pub pos: Vec2,
    /// A color, constructed by `rgb` values.
    pub color: Vec3,
    /// The texture coordinates.
    pub tex: Vec2,
}

impl Vertex {
    /// Creates a new vertex with the given `x`, `y` as a position, and `color` as the color.
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self {
            pos: Vec2::new(x, y),
            color: color.into(),
            ..Default::default()
        }
    }

    /// Creates a new vertex with the given `x` and `y` as a position
    /// as well as texture coordinates `s` and `t`
    pub fn with_tex(x: f32, y: f32, s: f32, t: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            tex: Vec2::new(s, t),
            ..Default::default()
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            pos: Vec2::new(0.0, 0.0),
            color: Color::WHITE.into(),
            tex: Vec2::new(0.0, 0.0),
        }
    }
}
