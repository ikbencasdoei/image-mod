use egui::{
    style::Margin, Align2, Button, Color32, Frame, LayerId, Order, Rounding, Sense, Stroke,
    TextStyle, Ui,
};
use uuid::Uuid;

use crate::{
    editor::Editor,
    modifier::{
        cation::{Cation, DynMod, Output},
        traits::{Modifier, ModifierIndex},
    },
};

#[derive(Clone, PartialEq)]
pub enum ModifierSlot {
    Modifier(Cation<DynMod>),
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
        Self::from_cacher(Cation::new(DynMod::new(modifier)))
    }

    pub fn from_cacher(cacher: Cation<DynMod>) -> Self {
        Self::Modifier(cacher)
    }

    pub fn from_index(index: &ModifierIndex) -> Self {
        Self::from_cacher(Cation::new(DynMod::from_index(index.clone())))
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }

    pub fn output<'a>(&'a mut self, input: &'a Output) -> &'a Output {
        if let Self::Modifier(content) = self {
            content.output(input)
        } else {
            input
        }
    }

    pub fn mod_ref(&self) -> Option<&Cation<DynMod>> {
        if let Self::Modifier(modifier) = self {
            Some(modifier)
        } else {
            None
        }
    }

    pub fn mod_mut(&mut self) -> Option<&mut Cation<DynMod>> {
        if let Self::Modifier(modifier) = self {
            Some(modifier)
        } else {
            None
        }
    }

    fn drag(&mut self) -> Option<Cation<DynMod>> {
        self.take(Self::Dragged(self.mod_ref()?.id))
    }

    fn take(&mut self, replacement: Self) -> Option<Cation<DynMod>> {
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

        ui.add_enabled_ui(false, |ui| ui.collapsing(&index.name, |_| {}));
    }

    fn view_modifier(&mut self, ui: &mut Ui, editor: &mut Editor, prefix: Option<&str>) {
        egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            ui.make_persistent_id(self.mod_ref().unwrap().id),
            true,
        )
        .show_header(ui, |ui| {
            if let Some(modifier) = self.mod_ref() {
                if let Some(text) = prefix {
                    ui.label(text);
                }

                if ui
                    .toggle_value(
                        &mut (editor.selected_id() == Some(modifier.id)),
                        &self.mod_ref().unwrap().modifier.index.name,
                    )
                    .clicked()
                {
                    editor.select_cation(modifier);
                }

                if ui
                    .add(Button::new("✋").sense(Sense::drag()))
                    .drag_started()
                {
                    editor.dragging = self.drag();
                }

                ui.menu_button("🗑", |ui| {
                    if ui.button("sure?").clicked() {
                        self.take(Self::Empty);
                        ui.close_menu();
                    }
                });
            }
        })
        .body(|ui| {
            if let Some(modifier) = self.mod_mut() {
                modifier.modifier.view(ui, editor)
            }
        });
    }

    fn view_slot(&mut self, ui: &mut Ui, editor: &mut Editor) {
        Frame {
            inner_margin: Margin::same(3.0),
            rounding: Rounding::same(3.0),
            stroke: Stroke::new(1.0, ui.style().visuals.text_color()),
            ..Default::default()
        }
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                if ui
                    .add(egui::Label::new("place here").sense(Sense::hover()))
                    .hovered()
                    && !ui.memory().is_anything_being_dragged()
                {
                    *self = Self::Modifier(editor.dragging.take().unwrap());
                }
            })
        });
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

    pub fn view_with_frame(&mut self, ui: &mut Ui, editor: &mut Editor, prefix: Option<&str>) {
        Frame {
            inner_margin: Margin::same(3.0),
            rounding: Rounding::same(3.0),
            stroke: Stroke::new(1.0, ui.style().visuals.text_color()),
            ..Default::default()
        }
        .show(ui, |ui| {
            if let Self::Empty = self {
                if editor.dragging.is_none() {
                    ui.horizontal(|ui| {
                        self.add_mod_widget(ui, editor);
                        ui.centered_and_justified(|ui| {
                            ui.label("empty");
                        })
                    });
                }
            }

            self.view(ui, editor, prefix);
        });
    }

    pub fn add_mod_widget(&mut self, ui: &mut Ui, editor: &mut Editor) {
        let mut text_edit_id = None;

        let inner = ui.menu_button("➕", |ui| {
            let response = ui.text_edit_singleline(&mut editor.add_mod_text);
            text_edit_id = Some(response.id);
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for index in editor.index.iter().filter(|index| {
                    editor.add_mod_text.is_empty()
                        || index
                            .name
                            .to_lowercase()
                            .contains(&editor.add_mod_text.to_lowercase())
                }) {
                    if ui.button(index.name.as_str()).clicked() {
                        ui.close_menu();
                        *self = ModifierSlot::from_index(index);
                    }
                }
            });
        });
        if inner.response.clicked() {
            if let Some(id) = text_edit_id {
                ui.memory().request_focus(id)
            }
        };
    }
}
