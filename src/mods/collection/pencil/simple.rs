use bevy::prelude::*;
use bevy_egui::egui::{Color32, Ui};

use crate::prelude::{Color, Image, *};

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
    pixels: Vec<UVec2>,
}

impl Default for SimplePencil {
    fn default() -> Self {
        Self {
            color: Color32::BLACK,
            pixels: default(),
        }
    }
}

impl Pencil for SimplePencil {
    fn add_pixel(&mut self, pixel: UVec2) {
        self.pixels.push(pixel);
    }
}

impl Modifier for SimplePencil {
    fn apply(&mut self, mut input: Option<Image>) -> Option<Image> {
        if let Some(image) = &mut input {
            for pixel in self.pixels.iter() {
                image.set_pixel(*pixel, Color::from(self.color)).ok();
            }
        }
        input
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.label("color");
        ui.color_edit_button_srgba(&mut self.color);
    }
}
