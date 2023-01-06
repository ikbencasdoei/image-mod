use bevy_egui::egui::{self, Ui};

use crate::prelude::{Image, *};

#[derive(Clone, Default, PartialEq)]
pub struct Blur {
    sigma: f32,
}

impl Modifier for Blur {
    fn apply(&mut self, mut input: Option<Image>) -> Option<Image> {
        if let Some(image) = &mut input {
            image.blur(self.sigma);
        }
        input
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("sigma:");
            ui.add(
                egui::DragValue::new(&mut self.sigma)
                    .speed(0.01)
                    .clamp_range(0.01..=f32::MAX),
            );
        });
    }
}
