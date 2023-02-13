use egui::{Color32, Ui};

use super::Pencil;
use crate::{color::Color, image::Image, position::Position};

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
    fn pixel(&mut self, _: Position, _: &mut Image) -> Option<Color> {
        Some(Color::from(self.color))
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("color:");
            ui.color_edit_button_srgba(&mut self.color);
        });
    }
}
