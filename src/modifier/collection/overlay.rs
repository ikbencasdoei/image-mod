use egui::{Context, Ui};
use glam::IVec2;

use crate::{
    editor::Editor,
    image::Image,
    modifier::{cation::Output, traits::Modifier},
    slot::ModifierSlot,
    view::View,
};

#[derive(Clone, PartialEq)]
pub struct Overlay {
    pub target: Option<IVec2>,
    pub input: ModifierSlot,
}

impl Default for Overlay {
    fn default() -> Self {
        Self {
            target: Default::default(),
            input: Default::default(),
        }
    }
}

impl Overlay {
    pub fn update(&mut self, ctx: &Context, view: &View) {
        if ctx.input().pointer.primary_clicked() && !ctx.wants_pointer_input() {
            if let Some(pos) = view.hovered_pixel(ctx) {
                let egui::Vec2 { x, y } = pos;
                self.target = Some(IVec2::new(x.round() as i32, y.round() as i32));
            }
        }
    }
}

impl Modifier for Overlay {
    fn apply(&mut self, mut input: Output) -> Option<Image> {
        if let Some(target) = self.target {
            if let Some(wrapped) = self.input.mod_mut() {
                if let Some(wrapped_output) = wrapped.output(&input).image.clone() {
                    if let Some(input) = &mut input.image {
                        input.overlay(&wrapped_output, target)
                    }
                }
            }
        }

        input.image
    }

    fn view(&mut self, ui: &mut Ui, editor: &mut Editor) {
        ui.label("input:");
        self.input.view_with_frame(ui, editor, None);

        if editor.is_modifier_selected::<Self>() {
            self.update(ui.ctx(), &editor.view);
        }
    }
}
