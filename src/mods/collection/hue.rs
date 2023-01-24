use egui::Ui;

use crate::{image::Image, mods::plugin::Modifier};

#[derive(Clone, Default, PartialEq)]
pub struct Hue {
    degrees: i32,
}

impl Modifier for Hue {
    fn apply(&mut self, mut input: Option<Image>) -> Option<Image> {
        if let Some(image) = &mut input {
            image.huerotate(self.degrees);
        }
        input
    }

    fn view(&mut self, ui: &mut Ui) {
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
