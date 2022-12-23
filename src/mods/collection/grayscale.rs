use bevy::prelude::Color as BevyColor;

use crate::prelude::{Color, Image, *};

#[derive(Clone, Default, PartialEq)]
pub struct GrayScaleFilter;

impl Modifier for GrayScaleFilter {
    fn apply(&mut self, mut input: Option<Image>) -> Option<Image> {
        if let Some(image) = &mut input {
            for position in image.coords().into_iter() {
                if let Ok(pixel) = image.get_pixel(position) {
                    let sum = pixel.sum() / 3.0;
                    image
                        .set_pixel(
                            position,
                            Color::from(BevyColor::rgba(sum, sum, sum, pixel.a())),
                        )
                        .ok();
                }
            }
        }
        input
    }
}
