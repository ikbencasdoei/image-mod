use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, Label, LayerId, Order, Sense, TextStyle, Ui},
    EguiContext,
};
use uuid::Uuid;

use super::collection::{ModifierCollectionPlugin, ModifierIndex};
use crate::{editor::Editor, ui::MenuBarSystemLabel};

pub struct ModifierUiPlugin;

impl Plugin for ModifierUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModifierUi>()
            .add_plugin(ModifierCollectionPlugin)
            .add_system(ui.after(MenuBarSystemLabel));
    }
}

#[derive(Resource, Default)]
pub struct ModifierUi {
    index: Vec<ModifierIndex>,
    dragging: Option<Uuid>,
}

impl ModifierUi {
    pub fn add_index(&mut self, index: ModifierIndex) {
        self.index.push(index);
        self.index.sort_by(|a, b| a.name.cmp(&b.name));
    }

    fn view(&mut self, editor: &mut Editor, ctx: &mut EguiContext) {
        egui::SidePanel::left("Modifiers")
            .resizable(true)
            .show(ctx.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(format!("Modifiers ({})", editor.get_mods().len()));
                    ui.separator();
                    self.mod_add_widget(ui, editor);
                });
                ui.separator();
                self.view_mods(editor, ui);
            });
    }

    fn mod_add_widget(&self, ui: &mut Ui, editor: &mut Editor) {
        ui.menu_button("add modifier", |ui| {
            for modifier in self.index.iter() {
                if ui.button(modifier.name.as_str()).clicked() {
                    editor.add_mod(modifier);
                    ui.close_menu();
                }
            }
        });
    }

    fn view_mods(&mut self, editor: &mut Editor, ui: &mut Ui) {
        if editor.get_mods().is_empty() {
            ui.label("(empty)");
        } else {
            let current_place = self.dragging.and_then(|id| editor.get_mod_index(id));

            for (i, id) in editor.mod_ids().into_iter().enumerate().rev() {
                if let Some(current_place) = current_place {
                    if current_place < i {
                        self.drop_mod_widget(i, editor, ui);
                    }
                }

                self.view_modifier(id, i, editor, ui);

                if let Some(current_place) = current_place {
                    if current_place >= i {
                        self.drop_mod_widget(i, editor, ui);
                    }
                }
            }

            if !ui.memory().is_anything_being_dragged() {
                self.dragging = None;
            }
        }
    }

    fn drop_mod_widget(&mut self, index: usize, editor: &mut Editor, ui: &mut Ui) {
        if self.dragging.is_some()
            && ui
                .add(egui::Label::new("place here").sense(Sense::hover()))
                .hovered()
            && !ui.memory().is_anything_being_dragged()
        {
            editor
                .mod_set_index(self.dragging.take().unwrap(), index)
                .unwrap()
        }
    }

    fn view_modifier(&mut self, mod_id: Uuid, index: usize, editor: &mut Editor, ui: &mut Ui) {
        let ui_id = ui.make_persistent_id(mod_id);

        if self.dragging.contains(&mod_id) {
            let layer = LayerId::new(Order::Tooltip, ui_id);

            if let Some(mouse_pos) = ui.ctx().pointer_interact_pos() {
                ui.ctx().layer_painter(layer).text(
                    mouse_pos,
                    Align2::CENTER_CENTER,
                    &editor.get_mod_mut(mod_id).unwrap().index.name,
                    TextStyle::Heading.resolve(ui.style()),
                    Color32::WHITE,
                );
            }
        } else {
            egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), ui_id, true)
                .show_header(ui, |ui| {
                    if ui
                        .add(Label::new(format!("#{index}")).sense(Sense::drag()))
                        .drag_started()
                    {
                        self.dragging = Some(mod_id);
                    }

                    if ui
                        .toggle_value(
                            &mut (editor.get_selected_mod_id() == Some(mod_id)),
                            editor.get_mod_mut(mod_id).unwrap().index.name.as_str(),
                        )
                        .clicked()
                    {
                        editor.select_mod(mod_id).unwrap();
                    }

                    ui.menu_button("remove", |ui| {
                        if ui.button("sure?").clicked() {
                            editor.remove_mod(mod_id).unwrap();
                            ui.close_menu();
                        }
                    });
                })
                .body(|ui| {
                    if let Some(modi) = editor.get_mod_mut(mod_id) {
                        modi.modifier.view(ui);
                    }
                });
        }
    }
}

fn ui(
    mut egui_context: ResMut<EguiContext>,
    mut editor: ResMut<Editor>,
    mut mod_ui: ResMut<ModifierUi>,
) {
    mod_ui.view(&mut editor, &mut egui_context);
}
