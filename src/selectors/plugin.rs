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
        Self: Sized + Default + Send + Sync + 'static,
    {
        SelectorIndex {
            name: type_name::<Self>().split("::").last().unwrap().to_string(),
            id: TypeId::of::<Self>(),
            instancer: Box::new(|| Box::new(Self::default())),
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
        app.add_startup_system(setup::<T>);
    }
}

fn setup<T: Selector + Default + Send + Sync + 'static>(
    mut collection: ResMut<SelectorCollection>,
) {
    collection.list.push(T::get_index());
}
