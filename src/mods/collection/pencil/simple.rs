use bevy::prelude::*;
use bevy_egui::egui::{Color32, Ui};

use crate::prelude::{Color, Image};

use super::plugin::{Pencil, PencilPlugin};

pub struct SimplePencilPlugin;

impl Plugin for SimplePencilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PencilPlugin::<SimplePencil>::default());
    }
}

#[derive(Clone, PartialEq)]
struct SimplePencil {
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
        ui.label("color");
        ui.color_edit_button_srgba(&mut self.color);
    }
}
