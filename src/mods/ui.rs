use std::any::TypeId;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::editor::Editor;

use crate::prelude::*;

pub struct ModifierCollectionPlugin;

impl Plugin for ModifierCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModifierCollection>()
            .add_plugin(ModifierPlugin::<GrayScaleFilter>::default())
            .add_system(ui);
    }
}

#[derive(PartialEq, Clone)]
pub struct ModifierIndex {
    pub name: String,
    pub id: TypeId,
}

#[derive(Resource, Default)]
pub struct ModifierCollection {
    pub list: Vec<ModifierIndex>,
}

fn ui(
    mut egui_context: ResMut<EguiContext>,
    collection: Res<ModifierCollection>,
    mut editor: ResMut<Editor>,
) {
    egui::Window::new("Modifiers").show(egui_context.ctx_mut(), |ui| {
        for modifier in collection.list.iter() {
            if ui
                .radio(
                    editor.selected_index == Some(modifier.to_owned()),
                    modifier.name.to_owned(),
                )
                .clicked()
            {
                if editor.selected_index == Some(modifier.to_owned()) {
                    editor.selected_index = None;
                } else {
                    editor.selected_index = Some(modifier.clone());
                }
            };
        }
    });
}
