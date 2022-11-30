use std::any::TypeId;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, CollapsingHeader},
    EguiContext,
};

use crate::editor::Editor;

use crate::prelude::*;

pub struct ModifierCollectionPlugin;

impl Plugin for ModifierCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModifierCollection>()
            .add_plugin(ModifierPlugin::<GrayScaleFilter>::default())
            .add_system(add_ui)
            .add_system(edit_ui);
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

fn add_ui(
    mut egui_context: ResMut<EguiContext>,
    collection: Res<ModifierCollection>,
    mut editor: ResMut<Editor>,
) {
    egui::Window::new("Add Modifier").show(egui_context.ctx_mut(), |ui| {
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
            }
        }
    });
}

fn edit_ui(mut egui_context: ResMut<EguiContext>, mut editor: ResMut<Editor>) {
    egui::Window::new("Modifiers").show(egui_context.ctx_mut(), |ui| {
        if editor.mods.is_empty() {
            ui.label("(empty)");
        } else {
            let mut remove_mod = None;
            for (index, modifier) in editor.mods.iter_mut().enumerate() {
                CollapsingHeader::new(modifier.index.name.as_str())
                    .default_open(false)
                    .id_source(modifier.id)
                    .show(ui, |ui| {
                        if ui.button("remove").clicked() {
                            remove_mod = Some(index);
                        }
                    });
            }

            if let Some(index) = remove_mod {
                editor.mods.remove(index);
            }
        }
    });
}
