use eframe::egui::Ui;

use crate::{
    editor::Editor,
    modifier::{cation::Output, traits::Modifier},
    position::Position,
    slot::ModifierSlot,
};

#[derive(Clone)]
pub struct Overlay {
    pub target: Position,
    pub input: ModifierSlot,
    dragging: bool,
}

impl Default for Overlay {
    fn default() -> Self {
        Self {
            target: Position::ZERO,
            input: Default::default(),
            dragging: false,
        }
    }
}

impl PartialEq for Overlay {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target && self.input == other.input
    }
}

impl Modifier for Overlay {
    fn apply(&mut self, input: &mut Output) {
        if let Some(wrapped) = self.input.mod_mut() {
            if let Some(wrapped_output) = wrapped.output(&input).image.clone() {
                if let Some(input) = &mut input.image {
                    input.overlay(&wrapped_output, self.target);
                }
            }
        }
    }

    fn view(&mut self, ui: &mut Ui, editor: &mut Editor) {
        ui.label("input:");
        self.input.view_with_frame(ui, editor, None);

        if editor.is_modifier_selected::<Self>() {
            if ui.rect_contains_pointer(editor.view.rect) {
                if ui
                    .ctx()
                    .input(|input| input.pointer.any_pressed() && input.pointer.primary_down())
                {
                    self.dragging = true;
                } else if ui.ctx().input(|input| input.pointer.any_released()) {
                    self.dragging = false;
                }
            }

            if self.dragging && ui.ctx().input(|input| input.pointer.any_down()) {
                self.target += Position::from(ui.ctx().input(|input| input.pointer.delta()))
                    * ui.ctx().pixels_per_point()
                    / editor.view.scale;
            } else {
                self.dragging = false;
            }
        }
    }
}
