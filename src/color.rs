use egui::Color32;

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

    pub fn into_rgba_u8(self) -> [u8; 4] {
        [
            (self.r * u8::MAX as f32) as u8,
            (self.g * u8::MAX as f32) as u8,
            (self.b * u8::MAX as f32) as u8,
            (self.a * u8::MAX as f32) as u8,
        ]
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let h = h.to_degrees();
        let c = v * s;
        let x = c * (1.0 - f32::abs((h / 60.0) % 2.0 - 1.0));
        let m = v - c;

        let (r, g, b) = match h as u32 % 360 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300..=u32::MAX => (c, 0.0, x),
        };

        Self::from_rgba(r + m, g + m, b + m, 1.0)
    }
}

impl From<Color32> for Color {
    fn from(color: Color32) -> Self {
        let (r, g, b, a) = color.to_tuple();

        Self::from_rgba_u8(r, g, b, a)
    }
}
