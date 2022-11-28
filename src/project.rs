use std::path::{Path, PathBuf};

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
    pub path: Option<PathBuf>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            input: DynamicImage::ImageRgb8(RgbImage::new(1, 1)),
            path: Default::default(),
        }
    }
}

impl Project {
    pub fn new_from_input_path(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        Ok(Self {
            input: image::open(path.as_ref())?,
            path: Some(path.as_ref().to_path_buf()),
        })
    }

    pub fn get_output(&self) -> DynamicImage {
        self.input.clone()
    }
}
