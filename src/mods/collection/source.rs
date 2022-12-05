use std::path::PathBuf;

use bevy::prelude::*;

use crate::prelude::{Image, *};

#[derive(Clone, Default)]
pub struct Source {
    pub path: PathBuf,
}

impl Modifier for Source {
    fn apply(&mut self, input: &mut Option<Image>, _: Vec<UVec2>) {
        *input = Image::open(&self.path).ok()
    }
}
