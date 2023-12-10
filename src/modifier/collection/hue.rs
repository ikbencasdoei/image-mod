use eframe::egui::{self, Ui};

use crate::{
    editor::Editor,
    modifier::{cation::Output, traits::Modifier},
};

#[derive(Clone, Default, PartialEq)]
pub struct Hue {
    degrees: i32,
}

impl Modifier for Hue {
    fn apply(&mut self, input: &mut Output) {
        if let Some(image) = &mut input.image {
            image.huerotate(self.degrees);
        }
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
