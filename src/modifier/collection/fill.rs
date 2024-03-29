use eframe::egui::{Color32, Ui};

use crate::{
    editor::Editor,
    modifier::{cation::Output, traits::Modifier},
};

#[derive(Clone, PartialEq)]
pub struct Fill {
    color: Color32,
}

impl Default for Fill {
    fn default() -> Self {
        Self {
            color: Color32::BLACK,
        }
    }
}

impl Modifier for Fill {
    fn apply(&mut self, input: &mut Output) {
        if let Some(image) = &mut input.image {
            for position in image.iter_coords() {
                image.set_pixel(position, self.color.into()).ok();
            }
        }
    }

    fn view(&mut self, ui: &mut Ui, _: &mut Editor) {
        ui.horizontal(|ui| {
            ui.label("color:");
            ui.color_edit_button_srgba(&mut self.color);
        });
    }
}
