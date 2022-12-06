use std::{
    any::{type_name, Any, TypeId},
    marker::PhantomData,
};

use bevy::prelude::*;
use dyn_clone::DynClone;

use crate::prelude::{Image, *};

pub trait Modifier: DynClone + DynPartialEq {
    fn apply(&mut self, input: Option<Image>, selection: Vec<UVec2>) -> Option<Image>;

    fn get_index() -> ModifierIndex
    where
        Self: Sized + Default + 'static,
    {
        ModifierIndex {
            name: type_name::<Self>().split("::").last().unwrap().to_string(),
            id: TypeId::of::<Self>(),
        }
    }
}

dyn_clone::clone_trait_object!(Modifier);

pub trait DynPartialEq {
    fn eq(&self, other: &dyn DynPartialEq) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl PartialEq for dyn DynPartialEq + '_ {
    fn eq(&self, other: &dyn DynPartialEq) -> bool {
        DynPartialEq::eq(self, other)
    }
}

impl<T: PartialEq + 'static> DynPartialEq for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq(&self, other: &dyn DynPartialEq) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.eq(other)
        } else {
            false
        }
    }
}

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
