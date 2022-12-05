use bevy::prelude::{Color as BevyColor, *};

use crate::prelude::{Color, Image, *};

#[derive(Clone, Default)]
pub struct GrayScaleFilter;

impl Modifier for GrayScaleFilter {
    fn apply(&mut self, input: &mut Option<Image>, selection: Vec<UVec2>) {
        if let Some(image) = input {
            for position in selection {
                if let Ok(pixel) = image.get_pixel(position) {
                    let sum = pixel.sum() / 4.0;
                    image
                        .set_pixel(position, Color::from(BevyColor::rgb(sum, sum, sum)))
                        .ok();
                }
            }
        }
    }
}
