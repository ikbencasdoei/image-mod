use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Ui},
    EguiContext,
};
use uuid::Uuid;

use crate::{editor::Editor, ui::MenuBarSystemLabel};

use super::{
    collection::{ModifierCollection, ModifierCollectionPlugin},
    modifier::Modification,
};

pub struct ModifierUiPlugin;

impl Plugin for ModifierUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ModifierCollectionPlugin)
            .add_system(ui.after(MenuBarSystemLabel));
    }
}

fn show_modifier(
    ui: &mut Ui,
    modification: &mut Modification,
    index: usize,
    selected: &mut Option<Uuid>,
) -> bool {
    let mut remove = false;
    let id = ui.make_persistent_id(modification.id);

    egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, true)
        .show_header(ui, |ui| {
            ui.label(format!("#{index}"));
            if ui
                .toggle_value(
                    &mut (*selected == Some(modification.id)),
                    modification.index.name.as_str(),
                )
                .clicked()
            {
                *selected = Some(modification.id);
            }
            ui.menu_button("remove", |ui| {
                if ui.button("sure?").clicked() {
                    remove = true;
                    ui.close_menu();
                }
            });
        })
        .body(|ui| {
            modification.modifier.view(ui);
        });

    remove
}

fn show_mods(ui: &mut Ui, editor: &mut Editor) {
    if editor.get_mods().is_empty() {
        ui.label("(empty)");
    } else {
        let mut remove_mod = None;
        let mut selected_mod = editor.get_selected_mod_id();
        for (i, modification) in editor.iter_mut_mods().enumerate().rev() {
            if show_modifier(ui, modification, i, &mut selected_mod) {
                remove_mod = Some(modification.id);
            }
        }

        if let Some(index) = remove_mod {
            editor.remove_mod(index);
        }

        if let Some(id) = selected_mod {
            editor.select_mod(id).ok();
        }
    }
}

fn ui(
    mut egui_context: ResMut<EguiContext>,
    mut editor: ResMut<Editor>,
    mut mod_collection: ResMut<ModifierCollection>,
) {
    let name = "Modifiers";

    egui::SidePanel::left(name)
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(format!("{name} ({})", editor.get_mods().len()));
                ui.separator();
                ui.menu_button("add modifier", |ui| {
                    mod_collection.list.sort_by(|a, b| a.name.cmp(&b.name));
                    for modifier in mod_collection.list.iter() {
                        if ui.button(modifier.name.as_str()).clicked() {
                            editor.add_mod(modifier);
                            ui.close_menu();
                        }
                    }
                })
            });
            ui.separator();
            show_mods(ui, &mut editor)
        });
}
