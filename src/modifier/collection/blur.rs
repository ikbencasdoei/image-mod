use egui::Ui;

use crate::{
    applied::AppliedValue,
    editor::Editor,
    modifier::{cation::Output, traits::Modifier},
};

#[derive(Clone, Default, PartialEq)]
pub struct Blur {
    sigma: AppliedValue<f32>,
}

impl Modifier for Blur {
    fn apply(&mut self, input: &mut Output) {
        if let Some(image) = &mut input.image {
            image.blur(*self.sigma);
        }
    }

    fn view(&mut self, ui: &mut Ui, _: &mut Editor) {
        ui.horizontal(|ui| {
            ui.label("sigma:");
            self.sigma.view(|value| {
                ui.add(
                    egui::DragValue::new(value)
                        .speed(0.01)
                        .clamp_range(0.01..=f32::MAX),
                )
            });
        });
    }
}
