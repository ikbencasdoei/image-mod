use std::marker::PhantomData;

use bevy::prelude::*;

use super::{collection::SelectorCollection, selection::Selection};

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
