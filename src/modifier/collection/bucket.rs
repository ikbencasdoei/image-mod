use egui::{Color32, Context, Ui};

use super::{fill::Fill, magic_wand::MagicWand};
use crate::{
    color::Color,
    editor::Editor,
    image::Image,
    modifier::{
        modification::{ModOutput, Modification},
        traits::Modifier,
    },
    view::View,
};

#[derive(Clone, PartialEq)]
pub struct Bucket {
    wand: MagicWand<Fill>,
}

impl Default for Bucket {
    fn default() -> Self {
        Self {
            wand: MagicWand {
                child: Some(Modification::new(Fill::default())),
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
    fn apply(&mut self, input: ModOutput) -> Option<Image> {
        self.wand.apply(input)
    }

    fn view(&mut self, ui: &mut Ui, editor: &mut Editor) {
        self.wand.child.as_mut().unwrap().modifier.view(ui, editor);

        ui.horizontal(|ui| {
            ui.label("threshold");
            ui.add(
                egui::DragValue::new(&mut self.wand.threshold)
                    .speed(0.001)
                    .clamp_range(0.0..=Color::from(Color32::WHITE).sum_rgb()),
            );
        });
    }
}
