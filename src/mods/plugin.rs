use std::{any::TypeId, marker::PhantomData};

use bevy::prelude::*;
use dyn_clone::DynClone;

use crate::prelude::{Color, Image, *};

pub trait Modifier: DynClone + Reflect {
    fn get_pixel(&mut self, position: UVec2, image: &mut Image) -> Option<Color>;
    fn get_index() -> ModifierIndex
    where
        Self: Sized + Default,
    {
        ModifierIndex {
            name: Self::default()
                .type_name()
                .split("::")
                .last()
                .unwrap()
                .to_string(),
            id: TypeId::of::<Self>(),
        }
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
        app.add_startup_system(setup::<T>).add_system(update::<T>);
    }
}

fn setup<T: Modifier + Default>(mut collection: ResMut<ModifierCollection>) {
    collection.list.push(T::get_index());
}

fn update<T: Modifier + Default>(
    mut editor: ResMut<Editor>,
    mut last: Local<Option<ModifierIndex>>,
) {
    if editor.selected_index != *last {
        if let Some(index) = editor.selected_index.clone() {
            if index.id == TypeId::of::<T>() {
                editor.receive_mod(T::default())
            }
        }
    }

    *last = editor.selected_index.clone();
}
