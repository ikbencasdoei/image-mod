use egui::Ui;
use uuid::Uuid;

use super::traits::{DynPartialEq, Modifier, ModifierIndex};
use crate::{editor::Editor, image::Image};

#[derive(Clone)]
pub struct Cation<T> {
    pub id: Uuid,
    pub modifier: T,
    cache: Option<Cache<T>>,
}

impl<T: Modifier + Clone + PartialEq> Cation<T> {
    pub fn new(modifier: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            modifier,
            cache: None,
        }
    }

    pub fn check_cache(&self, input: &Output) -> bool {
        self.cache
            .as_ref()
            .is_some_and(|cache| !cache.changed(&self.modifier) && cache.input_id == input.id)
    }

    pub fn output(&mut self, input: &Output) -> &Output {
        if self.check_cache(input) {
            return &self.cache.as_ref().unwrap().output;
        }

        self.apply(input)
    }

    fn apply(&mut self, input: &Output) -> &Output {
        let output = Output::new(self.modifier.apply(input.clone()));

        self.cache = Some(Cache {
            modifier: self.modifier.clone(),
            output,
            input_id: input.id,
        });

        &self.cache.as_ref().unwrap().output
    }
}

impl<T: PartialEq> PartialEq for Cation<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.modifier == other.modifier
    }
}

#[derive(Clone)]
pub struct DynMod {
    pub index: ModifierIndex,
    modifier: Box<dyn Modifier>,
}

impl DynMod {
    pub fn new<M>(modifier: M) -> Self
    where
        M: Modifier + Default + 'static,
    {
        Self {
            index: M::index(),
            modifier: Box::new(modifier),
        }
    }

    pub fn from_index(index: ModifierIndex) -> Self {
        Self {
            modifier: index.instancer.instance(),
            index: index,
        }
    }

    pub fn modifier<M: Modifier + Default + 'static>(&self) -> Option<&M> {
        if self.index == M::index() {
            let ptr: *const _ = &*self.modifier;
            unsafe { Some(&*ptr.cast()) }
        } else {
            None
        }
    }

    pub fn modifier_mut<M: Modifier + Default + 'static>(&mut self) -> Option<&mut M> {
        if self.index == M::index() {
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
    fn apply(&mut self, input: Output) -> Option<Image> {
        self.modifier.apply(input)
    }

    fn view(&mut self, ui: &mut Ui, editor: &mut Editor) {
        self.modifier.view(ui, editor);
    }
}

#[derive(Clone)]
pub struct Output {
    pub image: Option<Image>,
    id: Uuid,
}

impl Output {
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
struct Cache<T> {
    modifier: T,
    pub output: Output,
    input_id: Uuid,
}

impl<T: PartialEq> Cache<T> {
    fn changed(&self, modifier: &T) -> bool {
        self.modifier != *modifier
    }
}
