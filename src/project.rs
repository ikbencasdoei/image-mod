use std::path::Path;

use bevy::prelude::*;
use image::{DynamicImage, ImageError, RgbImage};

pub struct ProjectPlugin;

impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Project>();
    }
}

#[derive(Resource)]
pub struct Project {
    input: DynamicImage,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            input: DynamicImage::ImageRgb8(RgbImage::new(1, 1)),
        }
    }
}

impl Project {
    pub fn new_from_input_path(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        Ok(Self {
            input: image::open(path)?,
        })
    }

    pub fn get_output(&self) -> DynamicImage {
        self.input.clone()
    }
}
