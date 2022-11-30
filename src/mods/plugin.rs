use std::marker::PhantomData;

use bevy::prelude::*;

use crate::prelude::*;

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
