use std::path::PathBuf;

use bevy::prelude::*;

use crate::prelude::{Image, *};

#[derive(Clone, Default, PartialEq)]
pub struct Source {
    pub path: PathBuf,
}

impl Modifier for Source {
    fn apply(&mut self, _: Option<Image>, _: Vec<UVec2>) -> Option<Image> {
        Image::open(&self.path).ok()
    }
}
