use std::marker::PhantomData;

use bevy::prelude::*;

use dyn_clone::DynClone;

use crate::{color::Color, image::Image};

use super::{
    collection::{ModifierCollection, ModifierIndex},
    selectors::selection::Selection,
};

#[derive(Default)]
pub struct ModifierPlugin<T>(PhantomData<T>);

impl<T> Plugin for ModifierPlugin<T>
where
    T: Modifier + Default + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup::<T>);
    }
}

fn setup<T: Modifier + Default>(mut collection: ResMut<ModifierCollection>) {
    collection.list.push(T::get_index());
}

pub trait Modifier: DynClone + Reflect {
    fn get_pixel(&mut self, position: UVec2, image: &mut Image) -> Option<Color>;
    fn get_index() -> ModifierIndex
    where
        Self: Sized + Default,
    {
        ModifierIndex::from_type_name(Self::default().type_name())
    }
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
