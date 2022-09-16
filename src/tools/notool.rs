use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use super::plugin::{self, ToolPlugin};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ToolPlugin::<NoTool>::default())
            .add_system(help);
    }
}

#[derive(Component, Default, Reflect)]
pub struct NoTool;
impl plugin::Tool<NoTool> for NoTool {
    fn get_description() -> plugin::ToolDescription {
        plugin::ToolDescription {
            name: "Nothing".to_string(),
        }
    }
}

fn help(mut egui_context: ResMut<EguiContext>, query: Query<&NoTool>) {
    for _ in query.iter() {
        egui::Window::new("Help").show(egui_context.ctx_mut(), |ui| {
            ui.label("Use the middle mouse button to drag the image and the scroll wheel to zoom in and out.");
        });
    }
}
