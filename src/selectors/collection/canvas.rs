use bevy::prelude::*;

use crate::{image::Image, selectors::selection::Selection};

#[derive(Default, Reflect)]
pub struct CanvasSelection;

impl Selection for CanvasSelection {
    fn get_pixels(&self, image: &Image) -> Vec<UVec2> {
        let size = image.size();

        (0..(size.x * size.y))
            .map(|i| UVec2::new(i % size.x, i / size.x))
            .collect()
    }
}
