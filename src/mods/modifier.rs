use uuid::Uuid;

use crate::prelude::{Image, *};

pub struct Modification {
    pub index: ModifierIndex,
    pub id: Uuid,
    modifier: Box<dyn Modifier + Send + Sync>,
    selection: Vec<Selection>,
    cache: Option<Image>,
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
            cache: None,
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

    pub fn apply(&mut self, output: &mut Option<Image>) {
        if self.cache.is_some() {
            *output = self.cache.clone();
        } else {
            let mut state = dyn_clone::clone_box(&self.modifier);
            let mut selection = Vec::new();
            for selector in self.selection.iter() {
                selection.extend_from_slice(&selector.selector.get_pixels(output));
            }

            state.apply(output, selection);

            if output.is_some() {
                self.cache = output.clone();
            }
        }
    }

    pub fn get_selection(&self) -> &Vec<Selection> {
        &self.selection
    }

    pub fn remove_selection(&mut self, index: usize) {
        self.selection.remove(index);
        self.cache = None;
    }
}
