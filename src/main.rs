use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};

mod color;
mod image;
mod keybinds;
mod tools;
mod viewer;

fn main() {
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
        .add_plugin(viewer::Plugin)
        .add_plugin(keybinds::Plugin)
        .add_plugin(tools::Plugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut egui_settings: ResMut<EguiSettings>) {
    commands.spawn(Camera2dBundle::default());
    egui_settings.scale_factor = 1.5;
}
