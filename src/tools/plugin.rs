use std::marker::PhantomData;

use bevy::{prelude::*, reflect::GetTypeRegistration};
use bevy_egui::egui::util::id_type_map::TypeId;

use super::{ToolCollection, ToolEvent, ToolManagerLabel};

#[derive(Clone, PartialEq, Eq)]
pub struct ToolDescription {
    pub name: String,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ToolIndex {
    pub description: ToolDescription,
    pub type_id: TypeId,
}

pub trait Tool<T: 'static> {
    fn get_description() -> ToolDescription;
    fn get_index() -> ToolIndex {
        ToolIndex {
            description: Self::get_description(),
            type_id: TypeId::of::<T>(),
        }
    }
}

pub struct ToolPlugin<T: Tool<T> + 'static>(PhantomData<T>);

impl<T: Tool<T>> Default for ToolPlugin<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Component + Default + GetTypeRegistration + Tool<T> + 'static> bevy::prelude::Plugin
    for ToolPlugin<T>
{
    fn build(&self, app: &mut App) {
        app.register_type::<T>()
            .add_startup_system(setup::<T>)
            .add_system(events::<T>.before(ToolManagerLabel));
    }
}

fn setup<T: Tool<T> + 'static>(mut tool_collection: ResMut<ToolCollection>) {
    tool_collection.tools.push(T::get_index());
}

fn events<T: Component + Default + 'static>(
    mut commands: Commands,
    mut event_reader: EventReader<ToolEvent>,
    query: Query<Entity, With<Sprite>>,
) {
    for event in event_reader.iter() {
        match event {
            ToolEvent::Switched { from, to } => {
                if let Some(from) = from {
                    if from.type_id == TypeId::of::<T>() {
                        for entity in &query {
                            commands.entity(entity).remove::<T>();
                        }
                    }
                }
                if let Some(to) = to {
                    if to.type_id == TypeId::of::<T>() {
                        for entity in &query {
                            commands.entity(entity).insert(T::default());
                        }
                    }
                }
            }
        }
    }
}
