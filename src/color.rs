use bevy::prelude::{Color as BevyColor, *};
use bevy_egui::egui::Color32;

#[derive(Deref, DerefMut, Clone, Copy)]
pub struct Color(BevyColor);

impl From<Color32> for Color {
    fn from(color: Color32) -> Self {
        let (r, g, b, a) = color.to_tuple();
        Self(BevyColor::rgba_u8(r, g, b, a))
    }
}

impl From<Vec4> for Color {
    fn from(color: Vec4) -> Self {
        Self(BevyColor::from(color))
    }
}

impl From<[u8; 4]> for Color {
    fn from(color: [u8; 4]) -> Self {
        Self(BevyColor::rgba_u8(color[0], color[1], color[2], color[3]))
    }
}
