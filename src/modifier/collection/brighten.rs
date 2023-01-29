use egui::Ui;

use crate::{
    editor::Editor,
    image::Image,
    modifier::{modification::CacheOutput, traits::Modifier},
};

#[derive(Clone, Default, PartialEq)]
pub struct Brighten {
    value: i32,
}

impl Modifier for Brighten {
    fn apply(&mut self, mut input: CacheOutput) -> Option<Image> {
        if let Some(image) = &mut input.image {
            image.brighten(self.value);
        }
        input.image
    }

    fn view(&mut self, ui: &mut Ui, _: &mut Editor) {
        ui.horizontal(|ui| {
            ui.label("amount:");
            ui.add(egui::DragValue::new(&mut self.value));
        });
    }
}
