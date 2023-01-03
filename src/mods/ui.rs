use std::any::TypeId;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Frame, Ui},
    EguiContext,
};
use dyn_clone::DynClone;
use uuid::Uuid;

use crate::{editor::Editor, ui::MenuBarSystemLabel};

use crate::prelude::*;

use super::collection::color::ColorFilter;

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
            if modification.cache.is_some() {
                ui.label("cached");
            }

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

pub fn mods_ui(
    mut egui_context: ResMut<EguiContext>,
    mut editor: ResMut<Editor>,
    mod_collection: Res<ModifierCollection>,
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
            show_mods(ui, &mut editor)
        });
}
