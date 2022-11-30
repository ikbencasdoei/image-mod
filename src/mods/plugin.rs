use std::marker::PhantomData;

use bevy::prelude::*;
use dyn_clone::DynClone;

use crate::prelude::{Color, Image, *};

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