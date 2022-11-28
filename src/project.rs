use std::path::{Path, PathBuf};

use bevy::prelude::*;
use image::ImageError;

use crate::image::Image;

pub struct ProjectPlugin;

impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Project>();
    }
}

#[derive(Resource, Default)]
pub struct Project {
    pub input: Image,
    pub path: Option<PathBuf>,
}

impl Project {
    pub fn new_from_input_path(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        Ok(Self {
            input: Image::open(path.as_ref())?,
            path: Some(path.as_ref().to_path_buf()),
        })
    }

    pub fn get_output(&self) -> Image {
        self.input.clone()
    }
}
