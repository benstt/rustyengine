use glam::Vec2;

#[repr(C)]
pub struct Vertex {
    pub pos: Vec2,
}

impl Vertex {
    /// Creates a new vertex with the given x, y as a position.
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
        }
    }
}
