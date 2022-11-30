use bevy::prelude::{Color as BevyColor, *};

use crate::{color::Color, image::Image, mods::modifier::Modifier};

#[derive(Clone)]
pub struct GrayScaleFilter;

impl Modifier for GrayScaleFilter {
    fn get_pixel(&mut self, position: UVec2, image: &mut Image) -> Option<Color> {
        if let Ok(pixel) = image.get_pixel(position) {
            let sum = pixel.sum() / 4.0;
            Some(Color::from(BevyColor::rgb(sum, sum, sum)))
        } else {
            None
        }
    }
}
