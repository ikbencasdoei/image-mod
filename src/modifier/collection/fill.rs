use egui::{Color32, Ui};

use crate::{
    editor::Editor,
    image::Image,
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
    fn apply(&mut self, mut input: Output) -> Option<Image> {
        if let Some(image) = &mut input.image {
            for position in image.iter_coords() {
                image.set_pixel(position, self.color.into()).ok();
            }
        }

        input.image
    }

    fn view(&mut self, ui: &mut Ui, _: &mut Editor) {
        ui.horizontal(|ui| {
            ui.label("color:");
            ui.color_edit_button_srgba(&mut self.color);
        });
    }
}
