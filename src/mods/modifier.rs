use bevy::prelude::UVec2;
use uuid::Uuid;

use crate::prelude::{Image, *};

pub struct Modification {
    pub index: ModifierIndex,
    pub id: Uuid,
    modifier: Box<dyn Modifier + Send + Sync>,
    selection: Vec<Selection>,
}

impl Modification {
    pub fn new<M>(modifier: M) -> Self
    where
        M: Modifier + Default + Send + Sync + 'static,
    {
        Self {
            index: M::get_index(),
            id: Uuid::new_v4(),
            modifier: Box::new(modifier),
            selection: Vec::new(),
        }
    }

    pub fn add_selection<S>(&mut self, selection: S)
    where
        S: Selector + Default + Send + Sync + 'static,
    {
        self.selection.push(Selection {
            selector: Box::new(selection),
            index: S::get_index(),
        });
    }

    pub fn get_pixels(&self, image: &Option<Image>) -> Vec<UVec2> {
        let mut selection = Vec::new();
        for selector in self.selection.iter() {
            selection.extend_from_slice(&selector.selector.get_pixels(image));
        }
        selection
    }

    pub fn get_output(&mut self, inputs: &mut [&mut Modification]) -> Option<Image> {
        let (dependency, inputs) = if inputs.len() >= 1 {
            inputs.split_at_mut(1)
        } else {
            (inputs, &mut [] as &mut [&mut Modification])
        };

        let mut input = if let Some(modification) = dependency.get_mut(0) {
            modification.get_output(inputs)
        } else {
            None
        };

        let mut state = dyn_clone::clone_box(&self.modifier);
        let pixels = self.get_pixels(&input);
        state.apply(&mut input, pixels);

        input
    }

    pub fn get_selection(&self) -> &Vec<Selection> {
        &self.selection
    }

    pub fn remove_selection(&mut self, index: usize) {
        self.selection.remove(index);
    }
}
