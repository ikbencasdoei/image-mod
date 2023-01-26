use egui::Ui;

use crate::{image::Image, modifier::traits::Modifier};

#[derive(Clone, Default, PartialEq)]
pub struct Brighten {
    value: i32,
}

impl Modifier for Brighten {
    fn apply(&mut self, mut input: Option<Image>) -> Option<Image> {
        if let Some(image) = &mut input {
            image.brighten(self.value);
        }
        input
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("amount:");
            ui.add(egui::DragValue::new(&mut self.value));
        });
    }
}
