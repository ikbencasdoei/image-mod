use egui::Color32;
use glam::Vec4;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn sum_rgb(self) -> f32 {
        self.r + self.g + self.b
    }

    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / u8::MAX as f32,
            g: g as f32 / u8::MAX as f32,
            b: b as f32 / u8::MAX as f32,
            a: a as f32 / u8::MAX as f32,
        }
    }

    pub fn as_rgba_u8(self) -> [u8; 4] {
        [
            (self.r * u8::MAX as f32) as u8,
            (self.g * u8::MAX as f32) as u8,
            (self.b * u8::MAX as f32) as u8,
            (self.a * u8::MAX as f32) as u8,
        ]
    }
}

impl From<Color32> for Color {
    fn from(color: Color32) -> Self {
        let (r, g, b, a) = color.to_tuple();

        Self::from_rgba_u8(r, g, b, a)
    }
}

impl From<Vec4> for Color {
    fn from(color: Vec4) -> Self {
        Self::from_rgba(color.x, color.y, color.z, color.w)
    }
}
