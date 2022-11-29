use std::marker::PhantomData;

use bevy::{prelude::*, reflect::GetTypeRegistration};
use bevy_egui::egui::util::id_type_map::TypeId;

use crate::view::View;

use super::{CurrentTool, ToolCollection};

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
            .add_system(update::<T>);
    }
}

fn setup<T: Tool<T> + 'static>(mut tool_collection: ResMut<ToolCollection>) {
    tool_collection.tools.push(T::get_index());
}

fn update<T: Component + Default + 'static>(
    mut commands: Commands,
    query: Query<Entity, With<View>>,
    mut last_tool: Local<Option<ToolIndex>>,
    current_tool: Res<CurrentTool>,
) {
    if **current_tool != *last_tool {
        if let Some(from) = &*last_tool {
            if from.type_id == TypeId::of::<T>() {
                for entity in &query {
                    commands.entity(entity).remove::<T>();
                }
            }
        }
        if let Some(to) = &**current_tool {
            if to.type_id == TypeId::of::<T>() {
                for entity in &query {
                    commands.entity(entity).insert(T::default());
                }
            }
        }

        *last_tool = current_tool.clone();
    }
}
