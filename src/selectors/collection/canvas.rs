use bevy::prelude::*;

use crate::prelude::{Image, *};

#[derive(Default)]
pub struct CanvasSelection;

impl Selector for CanvasSelection {
    fn get_pixels(&self, image: &Option<Image>) -> Vec<UVec2> {
        if let Some(image) = image {
            let size = image.size();

            (0..(size.x * size.y))
                .map(|i| UVec2::new(i % size.x, i / size.x))
                .collect()
        } else {
            Vec::new()
        }
    }
}
