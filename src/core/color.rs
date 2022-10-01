use glam::Vec3;

/// A color. It is represented as 3 `rgb` values, without an alpha.
#[derive(Clone, Copy)]
pub struct Color {
    /// The amount of red.
    r: u8,
    /// The amount of green.
    g: u8,
    /// The amount of blue.
    b: u8,
}

impl Color {
    /// Constructs a new color with the given `rgb` values.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const RED: Color = Self::new(255, 0, 0);
    pub const GREEN: Color = Self::new(0, 255, 0);
    pub const BLUE: Color = Self::new(0, 0, 255);
    pub const YELLOW: Color = Self::new(255, 255, 0);
    pub const MAGENTA: Color = Self::new(255, 0, 255);
    pub const CYAN: Color = Self::new(0, 255, 255);
    pub const ORANGE: Color = Self::new(255, 128, 0);
    pub const LIGHTGREEN: Color = Self::new(128, 255, 0);
    pub const DARKGREEN: Color = Self::new(0, 255, 128);
    pub const LIGHTBLUE: Color = Self::new(0, 128, 255);
    pub const PURPLE: Color = Self::new(128, 0, 255);
    pub const PINK: Color = Self::new(255, 0, 128);
    pub const GREY: Color = Self::new(128, 128, 128);
    pub const BLACK: Color = Self::new(0, 0, 0);
    pub const WHITE: Color = Self::new(255, 255, 255);
}

impl Into<Vec3> for Color {
    fn into(self) -> Vec3 {
        Vec3::new(self.r as f32, self.g as f32, self.b as f32)
    }
}
