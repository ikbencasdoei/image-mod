use egui::{Align2, Color32, Label, LayerId, Order, Sense, TextStyle, Ui};
use uuid::Uuid;

use crate::{
    editor::Editor,
    image::Image,
    modifier::{
        modification::{DynMod, ModOutput, Modification},
        traits::{Modifier, ModifierIndex},
    },
};

#[derive(Default, Clone, PartialEq)]
pub struct List {
    pub contents: Vec<Modification<DynMod>>,
}

impl List {
    fn add_mod_button(&mut self, ui: &mut Ui, editor: &mut Editor) {
        ui.vertical_centered(|ui| {
            ui.menu_button("add modifier", |ui| {
                for index in editor.index.clone().iter() {
                    if ui.button(index.name.as_str()).clicked() {
                        self.add_mod_from_index(index, editor);
                        ui.close_menu();
                    }
                }
            });
        });

        ui.separator();
    }

    pub fn add_mod_from_index(&mut self, index: &ModifierIndex, editor: &mut Editor) {
        let new = Modification::new(DynMod::from_index(index.clone()));
        editor.selected = Some(new.id);
        self.contents.push(new);
    }

    pub fn remove_mod(&mut self, id: Uuid, editor: &mut Editor) -> Result<(), &str> {
        if let Some(index) = self.get_mod_index(id) {
            self.contents.remove(index);

            if let Some(selected) = editor.selected {
                if selected == id {
                    editor.selected = None;
                }
            }
            Ok(())
        } else {
            Err("invalid id")
        }
    }

    pub fn get_mod_index(&mut self, id: Uuid) -> Option<usize> {
        self.contents
            .iter()
            .enumerate()
            .find(|item| item.1.id == id)
            .map(|item| item.0)
    }

    pub fn mod_set_index(&mut self, id: Uuid, index: usize) -> Result<(), &str> {
        if let Some(i) = self.get_mod_index(id) {
            let modification = self.contents.remove(i);
            self.contents.insert(index, modification);
            Ok(())
        } else {
            Err("invalid id")
        }
    }

    pub fn get_selected_mod_mut(&mut self, editor: &Editor) -> Option<&mut Modification<DynMod>> {
        editor.selected.and_then(|id| self.get_mod_mut(id))
    }

    pub fn get_mod_mut(&mut self, id: Uuid) -> Option<&mut Modification<DynMod>> {
        self.contents.iter_mut().find(|item| item.id == id)
    }

    pub fn get_mods_of_type<T: Modifier + Default + 'static>(&self) -> Vec<&T> {
        self.contents
            .iter()
            .map(|modification| modification.modifier.get_modifier())
            .flatten()
            .collect()
    }

    fn view_modifier(
        index: usize,
        modification: &mut Modification<DynMod>,
        ui: &mut Ui,
        editor: &mut Editor,
    ) {
        egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            ui.make_persistent_id(modification.id),
            true,
        )
        .show_header(ui, |ui| {
            if ui
                .add(Label::new(format!("#{index}")).sense(Sense::drag()))
                .drag_started()
            {
                editor.dragging = Some(modification.id);
            }

            if ui
                .toggle_value(
                    &mut (editor.selected == Some(modification.id)),
                    &modification.modifier.index.name,
                )
                .clicked()
            {
                editor.selected = Some(modification.id);
            }

            ui.menu_button("remove", |ui| {
                if ui.button("sure?").clicked() {
                    editor.removed = Some(modification.id);
                    ui.close_menu();
                }
            });
        })
        .body(|ui| modification.modifier.view(ui, editor));
    }

    fn view_dragging(modification: &mut Modification<DynMod>, ui: &mut Ui) {
        let layer = LayerId::new(Order::Tooltip, ui.make_persistent_id(modification.id));
        if let Some(mouse_pos) = ui.ctx().pointer_interact_pos() {
            ui.ctx().layer_painter(layer).text(
                mouse_pos,
                Align2::CENTER_CENTER,
                &modification.modifier.index.name,
                TextStyle::Heading.resolve(ui.style()),
                Color32::WHITE,
            );
        }
    }

    pub fn drop_mod_widget(index: usize, ui: &mut Ui, editor: &mut Editor) {
        if ui
            .add(egui::Label::new("place here").sense(Sense::hover()))
            .hovered()
            && !ui.memory().is_anything_being_dragged()
        {
            editor.dropped = Some(index);
        }
    }
}

impl Modifier for List {
    fn apply(&mut self, mut output: ModOutput) -> Option<Image> {
        {
            let mut borrow = &output;
            for modification in self.contents.iter_mut() {
                borrow = modification.get_output(borrow);
            }
            output = borrow.clone();
        }

        output.image
    }

    fn view(&mut self, ui: &mut Ui, editor: &mut Editor) {
        self.add_mod_button(ui, editor);

        if self.contents.is_empty() {
            ui.label("(empty)");
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(id) = editor.dragging {
                    let current_place = self.get_mod_index(id).unwrap_or(0);
                    for (i, modification) in self.contents.iter_mut().enumerate().rev() {
                        if current_place < i {
                            Self::drop_mod_widget(i, ui, editor);
                        }

                        if id == modification.id {
                            Self::view_dragging(modification, ui);
                        } else {
                            Self::view_modifier(i, modification, ui, editor);
                        }

                        if current_place >= i {
                            Self::drop_mod_widget(i, ui, editor);
                        }
                    }
                } else {
                    for (i, modification) in self.contents.iter_mut().enumerate().rev() {
                        Self::view_modifier(i, modification, ui, editor);
                    }
                }

                if let Some(id) = editor.removed {
                    if self.remove_mod(id, editor).is_ok() {
                        editor.removed.take();
                    }
                }

                if let Some(index) = editor.dropped {
                    self.mod_set_index(editor.dragging.unwrap(), index).ok();
                    editor.dropped.take();
                }

                if !ui.memory().is_anything_being_dragged() {
                    editor.dragging.take();
                }
            });
        }
    }
}
