use std::{
    any::{type_name, TypeId},
    marker::PhantomData,
};

use bevy::prelude::*;

use crate::prelude::{Image, *};

pub trait Selector {
    fn get_pixels(&self, image: &Option<Image>) -> Vec<UVec2>;
    fn get_index() -> SelectorIndex
    where
        Self: Sized + Default + 'static,
    {
        SelectorIndex {
            name: type_name::<Self>().split("::").last().unwrap().to_string(),
            id: TypeId::of::<Self>(),
        }
    }
}

pub struct Selection {
    pub selector: Box<dyn Selector + Send + Sync + 'static>,
    pub index: SelectorIndex,
}

#[derive(Default)]
pub struct SelectorPlugin<T>(PhantomData<T>);

impl<T> Plugin for SelectorPlugin<T>
where
    T: Selector + Default + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup::<T>).add_system(update::<T>);
    }
}

fn setup<T: Selector + Default + 'static>(mut collection: ResMut<SelectorCollection>) {
    collection.list.push(T::get_index());
}

fn update<T: Selector + Default + Send + Sync + 'static>(
    mut editor: ResMut<Editor>,
    mut last: Local<Option<SelectorIndex>>,
) {
    if editor.add_sel_index != *last {
        if let Some(index) = editor.add_sel_index.clone() {
            if index.id == TypeId::of::<T>() {
                editor.receive_sel(T::get_index(), T::default())
            }
        }
    }

    *last = editor.add_sel_index.clone();
}
