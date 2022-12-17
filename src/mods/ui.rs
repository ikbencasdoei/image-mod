use std::any::TypeId;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Ui},
    EguiContext,
};
use dyn_clone::DynClone;
use uuid::Uuid;

use crate::{editor::Editor, ui::MenuBarSystemLabel};

use crate::prelude::*;

use super::collection::filters::color::ColorFilter;

pub struct ModifierCollectionPlugin;

impl Plugin for ModifierCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModifierCollection>()
            .add_plugin(ModifierPlugin::<GrayScaleFilter>::default())
            .add_plugin(ModifierPlugin::<Source>::default())
            .add_plugin(ModifierPlugin::<ColorFilter>::default())
            .add_system(mods_ui.after(MenuBarSystemLabel));
    }
}

pub trait ModInstancer: Fn() -> Box<dyn Modifier + Send + Sync + 'static> + DynClone {
    fn instance(&self) -> Box<dyn Modifier + Send + Sync>;
}

impl<T: Fn() -> Box<dyn Modifier + Send + Sync + 'static> + DynClone> ModInstancer for T {
    fn instance(&self) -> Box<dyn Modifier + Send + Sync> {
        self()
    }
}

dyn_clone::clone_trait_object!(ModInstancer);

#[derive(Clone)]
pub struct ModifierIndex {
    pub name: String,
    pub id: TypeId,
    pub instancer: Box<dyn ModInstancer + Send + Sync + 'static>,
}

impl PartialEq for ModifierIndex {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Resource, Default)]
pub struct ModifierCollection {
    pub list: Vec<ModifierIndex>,
}

fn show_selections(ui: &mut Ui, modification: &mut Modification, collection: &SelectorCollection) {
    let id = ui.make_persistent_id(modification.id);
    egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, true)
        .show_header(ui, |ui| {
            ui.label(format!(
                "selections ({})",
                modification.get_selection().len()
            ));
            ui.menu_button("add", |ui| {
                for index in collection.list.iter() {
                    if ui.button(index.name.to_owned()).clicked() {
                        modification.add_selection_from_index(index.clone());
                        ui.close_menu();
                    }
                }
            });
        })
        .body(|ui| {
            let mut remove_selection = None;
            for (index, selection) in modification.get_selection().iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(selection.index.name.as_str());
                    ui.menu_button("remove", |ui| {
                        if ui.button("sure?").clicked() {
                            remove_selection = Some(index);
                            ui.close_menu();
                        }
                    });
                });
            }

            if let Some(index) = remove_selection {
                modification.remove_selection(index);
            }
        });
}

fn show_modifier(
    ui: &mut Ui,
    modification: &mut Modification,
    index: usize,
    selected: &mut Option<Uuid>,
    sel_collection: &SelectorCollection,
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
            if modification.cache.is_some() {
                ui.label("cached");
            }

            modification.modifier.view(ui);

            show_selections(ui, modification, sel_collection);
        });

    remove
}

fn show_mods(ui: &mut Ui, editor: &mut Editor, sel_collection: &SelectorCollection) {
    if editor.get_mods().is_empty() {
        ui.label("(empty)");
    } else {
        let mut remove_mod = None;
        let mut selected_mod = editor.get_selected_mod();
        for (i, modification) in editor.iter_mut_mods().enumerate().rev() {
            if show_modifier(ui, modification, i, &mut selected_mod, sel_collection) {
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

pub fn mods_ui(
    mut egui_context: ResMut<EguiContext>,
    mut editor: ResMut<Editor>,
    mod_collection: Res<ModifierCollection>,
    sel_collection: Res<SelectorCollection>,
) {
    let name = "Modifiers";

    egui::SidePanel::left(name)
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(format!("{name} ({})", editor.get_mods().len()));
                ui.separator();
                ui.menu_button("add modifier", |ui| {
                    for modifier in mod_collection.list.iter() {
                        if ui.button(modifier.name.as_str()).clicked() {
                            editor.add_mod(modifier);
                            ui.close_menu();
                        }
                    }
                })
            });
            ui.separator();
            show_mods(ui, &mut editor, &sel_collection)
        });
}
