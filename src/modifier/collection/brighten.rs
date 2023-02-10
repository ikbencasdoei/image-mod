use egui::Ui;

use crate::{
    editor::Editor,
   
    modifier::{cation::Output, traits::Modifier},
};

#[derive(Clone, Default, PartialEq)]
pub struct Brighten {
    value: i32,
}

impl Modifier for Brighten {
    fn apply(&mut self, input: &mut Output) {
        if let Some(image) = &mut input.image {
            image.brighten(self.value);
        }
    }

    fn view(&mut self, ui: &mut Ui, _: &mut Editor) {
        ui.horizontal(|ui| {
            ui.label("amount:");
            ui.add(egui::DragValue::new(&mut self.value));
        });
    }
}
