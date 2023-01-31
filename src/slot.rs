use egui::{Align2, Color32, Label, LayerId, Order, Sense, TextStyle, Ui};
use uuid::Uuid;

use crate::{
    editor::Editor,
    modifier::{
        modification::{CacheOutput, Cacher, DynMod},
        traits::Modifier,
    },
};

#[derive(Clone, PartialEq)]
pub enum ModifierSlot {
    Modifier(Cacher<DynMod>),
    Dragged(Uuid),
    Empty,
}

impl Default for ModifierSlot {
    fn default() -> Self {
        Self::Empty
    }
}

impl ModifierSlot {
    pub fn from_mod<T: Modifier + Default + 'static>(modifier: T) -> Self {
        Self::from_cacher(Cacher::new(DynMod::new(modifier)))
    }

    pub fn from_cacher(cacher: Cacher<DynMod>) -> Self {
        Self::Modifier(cacher)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }

    pub fn get_output<'a>(&'a mut self, input: &'a CacheOutput) -> &'a CacheOutput {
        if let Self::Modifier(content) = self {
            content.get_output(input)
        } else {
            input
        }
    }

    pub fn get_mod(&self) -> Option<&Cacher<DynMod>> {
        if let Self::Modifier(modifier) = self {
            Some(modifier)
        } else {
            None
        }
    }

    pub fn get_mod_mut(&mut self) -> Option<&mut Cacher<DynMod>> {
        if let Self::Modifier(modifier) = self {
            Some(modifier)
        } else {
            None
        }
    }

    fn drag(&mut self) -> Option<Cacher<DynMod>> {
        self.take(Self::Dragged(self.get_mod()?.id))
    }

    fn take(&mut self, replacement: Self) -> Option<Cacher<DynMod>> {
        let Self::Modifier(modifier) = std::mem::replace(self, replacement) else {
               return None;
            };
        Some(modifier)
    }

    fn view_dragging(&mut self, ui: &mut Ui, editor: &mut Editor) {
        let index = &editor.dragging.as_ref().unwrap().modifier.index;

        let layer = LayerId::new(
            Order::Tooltip,
            ui.make_persistent_id(editor.dragging.as_ref().unwrap().id),
        );
        if let Some(mouse_pos) = ui.ctx().pointer_interact_pos() {
            ui.ctx().layer_painter(layer).text(
                mouse_pos,
                Align2::CENTER_CENTER,
                &index.name,
                TextStyle::Heading.resolve(ui.style()),
                Color32::WHITE,
            );
        }

        ui.add_enabled_ui(false, |ui| {
            ui.collapsing(&index.name, |ui| {
                ui.label("Body");
            })
        });
    }

    fn view_modifier(&mut self, ui: &mut Ui, editor: &mut Editor, prefix: Option<&str>) {
        egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            ui.make_persistent_id(self.get_mod().unwrap().id),
            true,
        )
        .show_header(ui, |ui| {
            let prefix = if let Some(prefix) = prefix {
                prefix
            } else {
                "✋"
            };

            if ui
                .add(Label::new(prefix).sense(Sense::drag()))
                .drag_started()
            {
                editor.dragging = self.drag();
            }

            if let Some(modifier) = self.get_mod() {
                if ui
                    .toggle_value(
                        &mut (editor.selected == Some(modifier.id)),
                        &self.get_mod().unwrap().modifier.index.name,
                    )
                    .clicked()
                {
                    editor.selected = Some(modifier.id);
                }

                ui.menu_button("remove", |ui| {
                    if ui.button("sure?").clicked() {
                        self.take(Self::Empty);
                        ui.close_menu();
                    }
                });
            }
        })
        .body(|ui| {
            if let Some(modifier) = self.get_mod_mut() {
                modifier.modifier.view(ui, editor)
            }
        });
    }

    fn view_slot(&mut self, ui: &mut Ui, editor: &mut Editor) {
        if ui
            .add(egui::Label::new("place here").sense(Sense::hover()))
            .hovered()
            && !ui.memory().is_anything_being_dragged()
        {
            *self = Self::Modifier(editor.dragging.take().unwrap());
        }
    }

    pub fn view(&mut self, ui: &mut Ui, editor: &mut Editor, prefix: Option<&str>) {
        match self {
            Self::Modifier(_) => self.view_modifier(ui, editor, prefix),
            Self::Empty => {
                if editor.dragging.is_some() {
                    self.view_slot(ui, editor);
                }
            }
            Self::Dragged(id) => {
                if editor
                    .dragging
                    .as_ref()
                    .is_some_and(|dragged| dragged.id == *id)
                {
                    self.view_dragging(ui, editor);
                } else {
                    if editor
                        .dropped
                        .as_ref()
                        .is_some_and(|dragged| dragged.id == *id)
                    {
                        *self = Self::Modifier(editor.dropped.take().unwrap());
                    } else {
                        self.take(Self::Empty);
                    }
                }
            }
        }
    }
}
