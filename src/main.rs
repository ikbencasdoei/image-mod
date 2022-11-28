use std::path::Path;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use keybinds::KeyBindsPlugin;
use project::{Project, ProjectPlugin};
use tools::ToolBoxPlugin;
use ui::UiPlugin;
use view::ViewPlugin;

mod color;
mod image_old;
mod keybinds;
mod project;
mod tools;
mod ui;
mod view;

fn main() {
    dotenvy::dotenv().ok();

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                fit_canvas_to_parent: true,
                title: env!("CARGO_PKG_NAME").to_string(),
                ..default()
            },
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(ProjectPlugin)
        .add_plugin(ViewPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(KeyBindsPlugin)
        .add_plugin(ToolBoxPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut egui_settings: ResMut<EguiSettings>, mut project: ResMut<Project>) {
    egui_settings.scale_factor = 1.5;

    if let Ok(path) = std::env::var("NEW_PROJECT_INPUT_PATH") {
        *project = Project::new_from_input_path(Path::new(&path)).unwrap()
    }
}
