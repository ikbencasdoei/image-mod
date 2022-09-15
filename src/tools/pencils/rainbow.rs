use bevy::{
    math::{Vec3Swizzles, Vec4Swizzles},
    prelude::{App, *},
};
use bevy_egui::{egui, EguiContext};

use crate::tools::plugin::{Tool, ToolDescription};

use super::plugin::{PencilPlugin, PencilTool};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PencilPlugin::<RainbowPencil>::default())
            .add_system(ui);
    }
}

#[derive(Component, Reflect)]
struct RainbowPencil {
    color_hsv: Vec3,
    rotation_per_pixel: f32,
    last_pixel: Option<Vec2>,
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

impl Tool<RainbowPencil> for RainbowPencil {
    fn get_description() -> ToolDescription {
        ToolDescription {
            name: "4: Rainbow Pencil".to_string(),
        }
    }
}

impl PencilTool for RainbowPencil {
    fn get_draw_color(&mut self, position: Vec2) -> [u8; 4] {
        let position = position.floor();
        let color = hsv2rgb(self.color_hsv);

        if let Some(last_pixel) = self.last_pixel {
            if position != last_pixel {
                self.color_hsv.x += self.rotation_per_pixel;
            }
        }

        self.last_pixel = Some(position);

        [
            (color.x * 255.0).round() as u8,
            (color.y * 255.0).round() as u8,
            (color.z * 255.0).round() as u8,
            u8::MAX,
        ]
    }
}

fn hsv2rgb(hsv: Vec3) -> Vec3 {
    let k = Vec4::from_array([1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0]);
    let p = ((hsv.xxx() + k.xyz()).fract() * 6.0 - k.www()).abs();
    return hsv.z
        * k.xxx()
            .lerp((p - k.xxx()).clamp(Vec3::ZERO, Vec3::ONE), hsv.y);
}

fn ui(mut egui_context: ResMut<EguiContext>, mut query: Query<&mut RainbowPencil>) {
    for mut pencil in query.iter_mut() {
        egui::Window::new("Rainbow Pencil").show(egui_context.ctx_mut(), |ui| {
            let mut degrees = pencil.color_hsv.x % 1.0 * 360.0;

            ui.label("current hue rotation:");
            ui.add(
                egui::DragValue::new(&mut degrees)
                    .speed(1.0)
                    .clamp_range(360.0..=0.0)
                    .suffix("°"),
            );

            let mut rotation_degrees = pencil.rotation_per_pixel * 360.0;

            ui.label("hue rotation per pixel:");
            ui.add(
                egui::DragValue::new(&mut rotation_degrees)
                    .speed(0.01)
                    .clamp_range(360.0..=0.0)
                    .suffix("°"),
            );

            pencil.color_hsv.x = degrees / 360.0;
            pencil.rotation_per_pixel = rotation_degrees / 360.0;
        });
    }
}
