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
            if ui.button(modifier.name.as_str()).clicked() {
                editor.add_mod(modifier)
            }
        }
    });
}

fn edit_ui(mut egui_context: ResMut<EguiContext>, mut editor: ResMut<Editor>) {
    egui::Window::new("Modifiers").show(egui_context.ctx_mut(), |ui| {
        if editor.get_mods().is_empty() {
            ui.label("(empty)");
        } else {
            let mut remove_mod = None;
            let mut selected_mod = editor.selected_mod;
            for modification in editor.iter_mut_mods() {
                let id = ui.make_persistent_id(modification.id);
                egui::collapsing_header::CollapsingState::load_with_default_open(
                    ui.ctx(),
                    id,
                    true,
                )
                .show_header(ui, |ui| {
                    if ui
                        .toggle_value(
                            &mut (selected_mod == Some(modification.id)),
                            modification.index.name.as_str(),
                        )
                        .clicked()
                    {
                        selected_mod = Some(modification.id);
                    }
                    if ui.button("remove").clicked() {
                        remove_mod = Some(modification.id);
                    }
                })
                .body(|ui| {
                    CollapsingHeader::new("selections")
                        .default_open(true)
                        .show(ui, |ui| {
                            let mut remove_selection = None;
                            for (index, selection) in
                                modification.get_selection().iter().enumerate()
                            {
                                ui.label(selection.index.name.as_str());
                                if ui.button("remove").clicked() {
                                    remove_selection = Some(index);
                                }
                            }

                            if let Some(index) = remove_selection {
                                modification.remove_selection(index);
                            }
                        });
                });
            }

            if let Some(index) = remove_mod {
                editor.remove_mod(index);
            }

            if let Some(index) = selected_mod {
                editor.selected_mod = Some(index);
            }
        }
    });
}
