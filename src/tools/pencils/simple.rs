use bevy::{prelude::*, reflect::FromReflect};
use bevy_egui::{egui, egui::Color32, EguiContext};

use crate::tools::plugin::{Tool, ToolDescription};

use super::plugin::{PencilPlugin, PencilTool};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PencilPlugin::<SimplePencil>::default())
            .add_system(input)
            .add_system(ui)
            .add_system(help);
    }
}

#[derive(Component, Reflect, FromReflect)]
pub struct SimplePencil {
    #[reflect(ignore)]
    primary_color: Color32,
    #[reflect(ignore)]
    secondary_color: Color32,
    #[reflect(ignore)]
    current_color: Color32,
}

impl Default for SimplePencil {
    fn default() -> Self {
        Self {
            primary_color: Color32::WHITE,
            secondary_color: Color32::BLACK,
            current_color: Color32::BLACK,
        }
    }
}

impl Tool<SimplePencil> for SimplePencil {
    fn get_description() -> ToolDescription {
        ToolDescription {
            name: "3: Simple Pencil".to_string(),
        }
    }
}

impl PencilTool for SimplePencil {
    fn get_draw_color(&mut self, _mouse_position: Vec2) -> [u8; 4] {
        self.current_color.to_array()
    }
}

fn input(mut query: Query<&mut SimplePencil>, mouse_button_input: Res<Input<MouseButton>>) {
    for mut pencil in query.iter_mut() {
        pencil.current_color = if mouse_button_input.pressed(MouseButton::Left) {
            pencil.primary_color
        } else {
            pencil.secondary_color
        }
    }
}

fn ui(mut query: Query<&mut SimplePencil>, mut egui_context: ResMut<EguiContext>) {
    for mut pencil in query.iter_mut() {
        egui::Window::new("Simple Pencil").show(egui_context.ctx_mut(), |ui| {
            ui.label("primary color:");
            ui.color_edit_button_srgba(&mut pencil.primary_color);
            ui.label("secondary color:");
            ui.color_edit_button_srgba(&mut pencil.secondary_color);
        });
    }
}

fn help(mut egui_context: ResMut<EguiContext>, query: Query<&SimplePencil>) {
    for _ in query.iter() {
        egui::Window::new("Simple Pencil Help").show(egui_context.ctx_mut(), |ui| {
            ui.label("Use the right mouse button to paint the primary color and use the left to paint the secondary color.");
        });
    }
}
