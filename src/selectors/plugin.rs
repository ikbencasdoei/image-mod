use std::{any::type_name, marker::PhantomData};

use bevy::prelude::*;

use crate::prelude::{Image, *};

pub trait Selection {
    fn get_pixels(&self, image: &Image) -> Vec<UVec2>;
    fn get_index() -> SelectorIndex
    where
        Self: Sized + Default,
    {
        SelectorIndex {
            name: type_name::<Self>().to_string(),
        }
    }
}

#[derive(Default)]
pub struct SelectorPlugin<T>(PhantomData<T>);

impl<T> Plugin for SelectorPlugin<T>
where
    T: Selection + Default + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup::<T>);
    }
}

fn setup<T: Selection + Default>(mut collection: ResMut<SelectorCollection>) {
    collection.list.push(T::get_index());
}
