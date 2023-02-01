use egui::{Context, Ui};

use super::{fill::Fill, magic_wand::MagicWand};
use crate::{
    editor::Editor,
    image::Image,
    modifier::{modification::Output, traits::Modifier},
    slot::ModifierSlot,
    view::View,
};

#[derive(Clone, PartialEq)]
pub struct Bucket {
    wand: MagicWand,
}

impl Default for Bucket {
    fn default() -> Self {
        Self {
            wand: MagicWand {
                input: ModifierSlot::from_mod(Fill::default()),
                ..Default::default()
            },
        }
    }
}

impl Bucket {
    pub fn update(&mut self, ctx: &Context, view: &View) {
        self.wand.update(ctx, view);
    }
}

impl Modifier for Bucket {
    fn apply(&mut self, input: Output) -> Option<Image> {
        self.wand.apply(input)
    }

    fn view(&mut self, ui: &mut Ui, editor: &mut Editor) {
        self.wand.input.mod_mut().unwrap().modifier.view(ui, editor);
        self.wand.view_threshold(ui);
    }
}
