use egui::{Color32, Ui};
use glam::UVec2;

use super::Pencil;
use crate::{color::Color, image::Image};

#[derive(Clone, PartialEq)]
pub struct SimplePencil {
    color: Color32,
}

impl Default for SimplePencil {
    fn default() -> Self {
        Self {
            color: Color32::BLACK,
        }
    }
}

impl Pencil for SimplePencil {
    fn get_pixel(&mut self, _: UVec2, _: &mut Image) -> Option<Color> {
        Some(Color::from(self.color))
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("color");
            ui.color_edit_button_srgba(&mut self.color);
        });
    }
}
