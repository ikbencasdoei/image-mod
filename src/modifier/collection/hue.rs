use egui::Ui;

use crate::{
    editor::Editor,
    image::Image,
    modifier::{modification::Output, traits::Modifier},
};

#[derive(Clone, Default, PartialEq)]
pub struct Hue {
    degrees: i32,
}

impl Modifier for Hue {
    fn apply(&mut self, mut input: Output) -> Option<Image> {
        if let Some(image) = &mut input.image {
            image.huerotate(self.degrees);
        }
        input.image
    }

    fn view(&mut self, ui: &mut Ui, _: &mut Editor) {
        ui.horizontal(|ui| {
            ui.label("degrees:");
            ui.add(
                egui::DragValue::new(&mut self.degrees)
                    .clamp_range(0..=360)
                    .suffix("°"),
            );
        });
    }
}
