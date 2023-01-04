use bevy::{
    math::{Vec3Swizzles, Vec4Swizzles},
    prelude::{App, *},
};
use bevy_egui::egui;

use crate::prelude::Color;

use super::plugin::{Pencil, PencilPlugin};

pub struct RainbowPencilPlugin;

impl Plugin for RainbowPencilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PencilPlugin::<RainbowPencil>::default());
    }
}

#[derive(Clone, PartialEq)]
struct RainbowPencil {
    color_hsv: Vec3,
    rotation_per_pixel: f32,
    last_pixel: Option<UVec2>,
}

impl Default for RainbowPencil {
    fn default() -> Self {
        Self {
            color_hsv: Vec3::new(0.0, 1.0, 1.0),
            rotation_per_pixel: 0.01,
            last_pixel: None,
        }
    }
}

impl Pencil for RainbowPencil {
    fn get_pixel(&mut self, pixel: UVec2) -> crate::prelude::Color {
        let color = hsv2rgb(self.color_hsv);

        if let Some(last_pixel) = self.last_pixel {
            if pixel != last_pixel {
                self.color_hsv.x += self.rotation_per_pixel;
            }
        }

        self.last_pixel = Some(pixel);

        Color::from(Vec4::from((color, 1.0)))
    }

    fn view(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                {
                    let mut degrees = self.color_hsv.x % 1.0 * 360.0;

                    ui.label("current hue rotation:");
                    ui.add(
                        egui::DragValue::new(&mut degrees)
                            .speed(1.0)
                            .clamp_range(360.0..=0.0)
                            .suffix("°"),
                    );
                    self.color_hsv.x = degrees / 360.0;
                }
                ui.end_row();

                {
                    let mut rotation_degrees = self.rotation_per_pixel * 360.0;

                    ui.label("hue rotation per pixel:");
                    ui.add(
                        egui::DragValue::new(&mut rotation_degrees)
                            .speed(0.01)
                            .clamp_range(360.0..=0.0)
                            .suffix("°"),
                    );

                    self.rotation_per_pixel = rotation_degrees / 360.0;
                }
                ui.end_row();
            });
    }
}

fn hsv2rgb(hsv: Vec3) -> Vec3 {
    let k = Vec4::from_array([1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0]);
    let p = ((hsv.xxx() + k.xyz()).fract() * 6.0 - k.www()).abs();
    hsv.z
        * k.xxx()
            .lerp((p - k.xxx()).clamp(Vec3::ZERO, Vec3::ONE), hsv.y)
}
