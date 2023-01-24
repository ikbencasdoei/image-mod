use egui::{Align2, Color32, Context, Label, LayerId, Order, Sense, TextStyle, Ui};
use uuid::Uuid;

use super::{collection::ModifierIndex, modifier::Modifier};
use crate::editor::Editor;

#[derive(Default)]
pub struct ModifierUi {
    index: Vec<ModifierIndex>,
    dragging: Option<Uuid>,
}

impl ModifierUi {
    pub fn add_index(&mut self, index: ModifierIndex) {
        self.index.push(index);
        self.index.sort_by(|a, b| a.name.cmp(&b.name));
    }

    pub fn view(&mut self, editor: &mut Editor, ctx: &Context) {
        egui::SidePanel::left("Modifiers")
            .resizable(true)
            .show(ctx, |ui| {
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
            egui::ScrollArea::vertical().show(ui, |ui| {
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
            });
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

    fn view_modifier(&mut self, id: Uuid, index: usize, editor: &mut Editor, ui: &mut Ui) {
        if self.dragging.contains(&id) {
            self.view_dragged_mod(id, editor, ui);
        } else {
            egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                ui.make_persistent_id(id),
                true,
            )
            .show_header(ui, |ui| {
                if ui
                    .add(Label::new(format!("#{index}")).sense(Sense::drag()))
                    .drag_started()
                {
                    self.dragging = Some(id);
                }

                if ui
                    .toggle_value(
                        &mut (editor.get_selected_mod_id() == Some(id)),
                        editor.get_mod_mut(id).unwrap().modifier.index.name.as_str(),
                    )
                    .clicked()
                {
                    editor.select_mod(id).unwrap();
                }

                ui.menu_button("remove", |ui| {
                    if ui.button("sure?").clicked() {
                        editor.remove_mod(id).unwrap();
                        ui.close_menu();
                    }
                });
            })
            .body(|ui| {
                if let Some(modi) = editor.get_mod_mut(id) {
                    modi.modifier.view(ui);
                }
            });
        }
    }

    fn view_dragged_mod(&self, id: Uuid, editor: &Editor, ui: &mut Ui) {
        let layer = LayerId::new(Order::Tooltip, ui.make_persistent_id(id));
        if let Some(mouse_pos) = ui.ctx().pointer_interact_pos() {
            ui.ctx().layer_painter(layer).text(
                mouse_pos,
                Align2::CENTER_CENTER,
                &editor.get_mod(id).unwrap().modifier.index.name,
                TextStyle::Heading.resolve(ui.style()),
                Color32::WHITE,
            );
        }
    }
}