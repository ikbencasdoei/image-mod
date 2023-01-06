use bevy_egui::egui::{self, Ui};

use crate::prelude::{Image, *};

#[derive(Clone, Default, PartialEq)]
pub struct Contrast {
    value: f32,
}

impl Modifier for Contrast {
    fn apply(&mut self, mut input: Option<Image>) -> Option<Image> {
        if let Some(image) = &mut input {
            image.contrast(self.value);
        }
        input
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("amount:");
            ui.add(egui::DragValue::new(&mut self.value).clamp_range(-100.0..=f32::MAX));
        });
    }
}
