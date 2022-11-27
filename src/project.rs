use std::path::Path;

use bevy::prelude::*;
use image::{DynamicImage, ImageError};

pub struct ProjectPlugin;

impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProjectMgr>();
    }
}

#[derive(Resource, Default)]
pub struct ProjectMgr {
    pub current: Option<Project>,
}

pub struct Project {
    input: DynamicImage,
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
