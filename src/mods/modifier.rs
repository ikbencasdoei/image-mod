use uuid::Uuid;

use crate::prelude::{Image, *};

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
            index: index,
            id: Uuid::new_v4(),
            modifier,
            cache: None,
        }
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
        let output = state.apply(input);
        self.cache = Some(ModCache {
            modifier: dyn_clone::clone(&self.modifier),
            image: output.clone(),
            last_input,
        });
        output
    }

    // pub fn get_modifier<T: Modifier + Default + Send + Sync + 'static>(&self) -> Option<&T> {
    //     if self.index == T::get_index() {
    //         unsafe {
    //             let ptr: *const _ = &*self.modifier;
    //             Some(&*ptr.cast())
    //         }
    //     } else {
    //         None
    //     }
    // }

    pub fn get_modifier_mut<T: Modifier + Default + Send + Sync + 'static>(
        &mut self,
    ) -> Option<&mut T> {
        if self.index == T::get_index() {
            unsafe {
                let ptr: *mut _ = &mut *self.modifier;
                Some(&mut *ptr.cast())
            }
        } else {
            None
        }
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
