use std::{
    any::{type_name, TypeId},
    marker::PhantomData,
};

use bevy::prelude::*;
use dyn_clone::DynClone;

use crate::prelude::{Image, *};

pub trait Modifier: DynClone {
    fn apply(&mut self, input: &mut Option<Image>, selection: Vec<UVec2>);

    fn get_index() -> ModifierIndex
    where
        Self: Sized + Default + 'static,
    {
        ModifierIndex {
            name: type_name::<Self>().split("::").last().unwrap().to_string(),
            id: TypeId::of::<Self>(),
        }
    }

    fn changed(&self) -> bool;
}

dyn_clone::clone_trait_object!(Modifier);

#[derive(Default)]
pub struct ModifierPlugin<T>(PhantomData<T>);

impl<T> Plugin for ModifierPlugin<T>
where
    T: Modifier + Default + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup::<T>).add_system(update::<T>);
    }
}

fn setup<T: Modifier + Default + 'static>(mut collection: ResMut<ModifierCollection>) {
    collection.list.push(T::get_index());
}

fn update<T: Modifier + Default + Send + Sync + 'static>(
    mut editor: ResMut<Editor>,
    mut last: Local<Option<ModifierIndex>>,
) {
    if editor.add_mod_index != *last {
        if let Some(index) = editor.add_mod_index.clone() {
            if index.id == TypeId::of::<T>() {
                editor.receive_mod(T::get_index(), T::default())
            }
        }
    }

    *last = editor.add_mod_index.clone();
}
