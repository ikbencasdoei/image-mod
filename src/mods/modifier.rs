use bevy::prelude::UVec2;
use uuid::Uuid;

use crate::prelude::{Image, *};

pub struct Modification {
    pub index: ModifierIndex,
    pub id: Uuid,
    pub modifier: Box<dyn Modifier + Send + Sync>,
    selection: Vec<Selection>,
    pub cache: Option<ModCache>,
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
        self.cache = None;
    }

    pub fn get_pixels(&self, image: &Option<Image>) -> Vec<UVec2> {
        let mut selection = Vec::new();
        for selector in self.selection.iter() {
            selection.extend_from_slice(&selector.selector.get_pixels(image));
        }
        selection
    }

    pub fn get_output(&mut self, inputs: &mut [&mut Modification]) -> ModOutput {
        let (dependency, inputs) = if inputs.len() >= 1 {
            inputs.split_at_mut(1)
        } else {
            (inputs, &mut [] as &mut [&mut Modification])
        };

        let modification = dependency.get_mut(0);

        let input = if let Some(modification) = modification {
            modification.get_output(inputs)
        } else {
            ModOutput::NoOutput
        };

        if let Some(cache) = &self.cache {
            if !cache.changed(&*self.modifier) {
                if cache.last_input == input.is_some() {
                    if let ModOutput::Cached(_) | ModOutput::NoOutput = input {
                        return ModOutput::Cached(cache.image.clone());
                    }
                }
            }
        }

        ModOutput::Modified(self.apply(input.get_output()))
    }

    fn apply(&mut self, input: Option<Image>) -> Option<Image> {
        let last_input = input.is_some();
        let mut state = dyn_clone::clone(&self.modifier);
        let pixels = self.get_pixels(&input);
        let output = state.apply(input, pixels);
        self.cache = Some(ModCache {
            modifier: dyn_clone::clone(&self.modifier),
            image: output.clone(),
            last_input,
        });
        output
    }

    pub fn get_selection(&self) -> &Vec<Selection> {
        &self.selection
    }

    pub fn remove_selection(&mut self, index: usize) {
        self.selection.remove(index);
        self.cache = None;
    }
}

pub enum ModOutput {
    Modified(Option<Image>),
    Cached(Option<Image>),
    NoOutput,
}

impl ModOutput {
    pub fn get_output(self) -> Option<Image> {
        match self {
            ModOutput::Modified(option) => option,
            ModOutput::Cached(option) => option,
            ModOutput::NoOutput => None,
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            ModOutput::Modified(option) => option.is_some(),
            ModOutput::Cached(option) => option.is_some(),
            ModOutput::NoOutput => false,
        }
    }
}

pub struct ModCache {
    modifier: Box<dyn Modifier + Send + Sync>,
    image: Option<Image>,
    last_input: bool,
}

impl ModCache {
    fn changed(&self, modifier: &dyn Modifier) -> bool {
        !self.modifier.eq(modifier as &dyn DynPartialEq)
    }
}
