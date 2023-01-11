use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, Label, LayerId, Order, Sense, TextStyle, Ui},
    EguiContext,
};
use uuid::Uuid;

use super::{
    collection::{ModifierCollection, ModifierCollectionPlugin},
    modifier::Modification,
};
use crate::{editor::Editor, ui::MenuBarSystemLabel};

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
    dragging: &mut Option<Uuid>,
) -> bool {
    let mut remove = false;
    let id = ui.make_persistent_id(modification.id);

    if dragging.contains(&modification.id) {
        let layer = LayerId::new(Order::Tooltip, id);

        if let Some(mouse_pos) = ui.ctx().pointer_interact_pos() {
            ui.ctx().layer_painter(layer).text(
                mouse_pos,
                Align2::CENTER_CENTER,
                &modification.index.name,
                TextStyle::Heading.resolve(ui.style()),
                Color32::WHITE,
            );
        }
    } else {
        egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| {
                if ui
                    .add(Label::new(format!("#{index}")).sense(Sense::drag()))
                    .drag_started()
                {
                    *dragging = Some(modification.id);
                }

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
    }

    remove
}

fn show_mods(ui: &mut Ui, editor: &mut Editor, dragging: &mut Option<Uuid>) {
    if editor.get_mods().is_empty() {
        ui.label("(empty)");
    } else {
        let mut remove_mod = None;
        let mut selected_mod = editor.get_selected_mod_id();
        let mut place_mod = None;
        let current_place = dragging.and_then(|id| editor.get_mod_index(id));
        for (i, modification) in editor.iter_mut_mods().enumerate().rev() {
            {
                if dragging.is_some()
                    && current_place.is_some()
                    && current_place.unwrap() < i
                    && ui
                        .add(egui::Label::new("place here").sense(Sense::hover()))
                        .hovered()
                    && !ui.memory().is_anything_being_dragged()
                {
                    place_mod = Some(i);
                }
            }

            if show_modifier(ui, modification, i, &mut selected_mod, dragging) {
                remove_mod = Some(modification.id);
            }

            {
                if dragging.is_some()
                    && current_place.is_some()
                    && current_place.unwrap() >= i
                    && ui
                        .add(egui::Label::new("place here").sense(Sense::hover()))
                        .hovered()
                    && !ui.memory().is_anything_being_dragged()
                {
                    place_mod = Some(i);
                }
            }
        }

        if let Some(index) = remove_mod {
            editor.remove_mod(index).ok();
        }

        if let Some(id) = selected_mod {
            editor.select_mod(id).ok();
        }

        if let Some(i) = place_mod {
            editor.mod_set_index(dragging.take().unwrap(), i).unwrap()
        }

        if !ui.memory().is_anything_being_dragged() {
            *dragging = None;
        }
    }
}

fn ui(
    mut egui_context: ResMut<EguiContext>,
    mut editor: ResMut<Editor>,
    mut mod_collection: ResMut<ModifierCollection>,
    mut dragging: Local<Option<Uuid>>,
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
            show_mods(ui, &mut editor, &mut dragging)
        });
}
