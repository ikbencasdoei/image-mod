use bevy_egui::egui::Ui;
use uuid::Uuid;

use super::{
    collection::ModifierIndex,
    plugin::{DynPartialEq, Modifier},
};
use crate::image::Image;

#[derive(Clone)]
pub struct Modification<T> {
    pub id: Uuid,
    pub modifier: T,
    pub cache: Option<ModCache<T>>,
}

impl<T: Modifier + Clone + PartialEq> Modification<T> {
    pub fn new(modifier: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            modifier,
            cache: None,
        }
    }

    pub fn check_cache(&self, input: &ModOutput) -> bool {
        self.cache
            .as_ref()
            .is_some_and(|cache| !cache.changed(&self.modifier) && cache.input_id == input.id)
    }

    pub fn get_output(&mut self, input: &ModOutput) -> &ModOutput {
        if self.check_cache(input) {
            return &self.cache.as_ref().unwrap().output;
        }

        self.apply(input)
    }

    fn apply(&mut self, input: &ModOutput) -> &ModOutput {
        let output = ModOutput::new(self.modifier.apply(input.image.clone()));

        self.cache = Some(ModCache {
            modifier: self.modifier.clone(),
            output,
            input_id: input.id,
        });

        &self.cache.as_ref().unwrap().output
    }
}

impl<T: PartialEq> PartialEq for Modification<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.modifier == other.modifier
    }
}

#[derive(Clone)]
pub struct DynMod {
    pub index: ModifierIndex,
    modifier: Box<dyn Modifier + Send + Sync>,
}

impl DynMod {
    pub fn new<M>(modifier: M) -> Self
    where
        M: Modifier + Default + Send + Sync + 'static,
    {
        Self {
            index: M::get_index(),
            modifier: Box::new(modifier),
        }
    }

    pub fn from_index(index: ModifierIndex) -> Self {
        Self {
            modifier: index.instancer.instance(),
            index: index,
        }
    }

    pub fn get_modifier<M: Modifier + Default + Send + Sync + 'static>(&self) -> Option<&M> {
        if self.index == M::get_index() {
            let ptr: *const _ = &*self.modifier;
            unsafe { Some(&*ptr.cast()) }
        } else {
            None
        }
    }

    pub fn get_modifier_mut<M: Modifier + Default + Send + Sync + 'static>(
        &mut self,
    ) -> Option<&mut M> {
        if self.index == M::get_index() {
            let ptr: *mut _ = &mut *self.modifier;
            unsafe { Some(&mut *ptr.cast()) }
        } else {
            None
        }
    }
}

impl PartialEq for DynMod {
    fn eq(&self, other: &Self) -> bool {
        &*self.modifier as &dyn DynPartialEq == &*other.modifier as &dyn DynPartialEq
    }
}

impl Modifier for DynMod {
    fn apply(&mut self, input: Option<Image>) -> Option<Image> {
        self.modifier.apply(input)
    }

    fn view(&mut self, ui: &mut Ui) {
        self.modifier.view(ui)
    }
}

#[derive(Clone)]
pub struct ModOutput {
    pub image: Option<Image>,
    id: Uuid,
}

impl ModOutput {
    pub fn new(image: Option<Image>) -> Self {
        Self {
            image,
            id: Uuid::new_v4(),
        }
    }

    pub fn new_empty() -> Self {
        Self {
            image: None,
            id: Uuid::nil(),
        }
    }
}

#[derive(Clone)]
pub struct ModCache<T> {
    modifier: T,
    pub output: ModOutput,
    input_id: Uuid,
}

impl<T: PartialEq> ModCache<T> {
    fn changed(&self, modifier: &T) -> bool {
        self.modifier != *modifier
    }
}
