use uuid::Uuid;

use super::{
    collection::ModifierIndex,
    plugin::{DynPartialEq, Modifier},
};
use crate::image::Image;

pub struct Modification {
    pub index: ModifierIndex,
    pub id: Uuid,
    pub modifier: Box<dyn Modifier + Send + Sync>,
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
            cache: None,
        }
    }

    pub fn new_from_index(index: ModifierIndex) -> Self {
        let modifier = index.instancer.instance();

        Self {
            index,
            id: Uuid::new_v4(),
            modifier,
            cache: None,
        }
    }

    pub fn get_output(&mut self, input: &ModOutput) -> &ModOutput {
        if let Some(cache) = &self.cache {
            if !cache.changed(&*self.modifier) && cache.input_id == input.id {
                return &cache.output;
            }
        }

        self.apply(input)
    }

    fn apply(&mut self, input: &ModOutput) -> &ModOutput {
        let output = ModOutput {
            image: self.modifier.apply(input.image.clone()),
            id: Uuid::new_v4(),
        };

        self.cache = Some(ModCache {
            modifier: dyn_clone::clone(&self.modifier),
            output,
            input_id: input.id,
        });

        &self.cache.as_ref().unwrap().output
    }

    #[allow(dead_code)]
    pub fn get_modifier<T: Modifier + Default + Send + Sync + 'static>(&self) -> Option<&T> {
        if self.index == T::get_index() {
            let ptr: *const _ = &*self.modifier;
            unsafe { Some(&*ptr.cast()) }
        } else {
            None
        }
    }

    pub fn get_modifier_mut<T: Modifier + Default + Send + Sync + 'static>(
        &mut self,
    ) -> Option<&mut T> {
        if self.index == T::get_index() {
            let ptr: *mut _ = &mut *self.modifier;
            unsafe { Some(&mut *ptr.cast()) }
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct ModOutput {
    pub image: Option<Image>,
    id: Uuid,
}

impl ModOutput {
    pub fn new_empty() -> Self {
        Self {
            image: None,
            id: Uuid::nil(),
        }
    }
}

pub struct ModCache {
    modifier: Box<dyn Modifier + Send + Sync>,
    pub output: ModOutput,
    input_id: Uuid,
}

impl ModCache {
    fn changed(&self, modifier: &dyn Modifier) -> bool {
        !self.modifier.eq(modifier as &dyn DynPartialEq)
    }
}
