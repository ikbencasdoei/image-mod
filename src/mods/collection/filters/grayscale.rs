use bevy::prelude::{Color as BevyColor, *};

use crate::prelude::{Color, Image, *};

#[derive(Clone, Default)]
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

    fn changed(&self) -> bool {
        false
    }
}
