use bevy::prelude::*;

use dyn_clone::DynClone;

use crate::{color::Color, image::Image};

use super::selection::Selection;

pub trait Modifier: DynClone {
    fn get_pixel(&mut self, position: UVec2, image: &mut Image) -> Option<Color>;
}

dyn_clone::clone_trait_object!(Modifier);

pub struct Modification {
    modifier: Box<dyn Modifier + Send + Sync>,
    selection: Vec<Box<dyn Selection + Send + Sync>>,
}

impl Modification {
    pub fn new<M>(modifier: M) -> Self
    where
        M: Modifier + Send + Sync + 'static,
    {
        Self {
            modifier: Box::new(modifier),
            selection: Vec::new(),
        }
    }

    pub fn add_selection<S>(&mut self, selection: S)
    where
        S: Selection + Send + Sync + 'static,
    {
        self.selection.push(Box::new(selection));
    }

    pub fn apply(&self, mut output: &mut Image) {
        let mut modifier_state = dyn_clone::clone_box(&self.modifier);
        for selection in self.selection.iter() {
            for position in selection.get_pixels(&output) {
                if let Some(color) = modifier_state.get_pixel(position, &mut output) {
                    output.set_pixel(position, color).unwrap();
                }
            }
        }
    }
}
