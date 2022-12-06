use bevy::prelude::{Color as BevyColor, *};

use crate::prelude::{Color, Image, *};

#[derive(Clone, Default, PartialEq)]
pub struct GrayScaleFilter;

impl Modifier for GrayScaleFilter {
    fn apply(&mut self, mut input: Option<Image>, selection: Vec<UVec2>) -> Option<Image> {
        if let Some(image) = &mut input {
            for position in selection {
                if let Ok(pixel) = image.get_pixel(position) {
                    let sum = pixel.sum() / 4.0;
                    image
                        .set_pixel(position, Color::from(BevyColor::rgb(sum, sum, sum)))
                        .ok();
                }
            }
        }
        input
    }
}
