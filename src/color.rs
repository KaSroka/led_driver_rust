#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const RED: Self = Self::from_rgb(255, 0, 0);
    pub const GREEN: Self = Self::from_rgb(0, 255, 0);
    pub const BLUE: Self = Self::from_rgb(0, 0, 255);
    pub const WHITE: Self = Self::from_rgb(255, 255, 255);

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn with_intensity(&self, intensity: f32) -> Self {
        Self {
            r: (self.r as f32 * intensity).round() as u8,
            g: (self.g as f32 * intensity).round() as u8,
            b: (self.b as f32 * intensity).round() as u8,
        }
    }
}