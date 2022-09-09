use bevy::{app::AppExit, prelude::*};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(fullscreen).add_system(exit);
    }
}

fn fullscreen(mut windows: ResMut<Windows>, key: Res<Input<KeyCode>>) {
    if key.just_pressed(KeyCode::F11) {
        if let Some(window) = windows.get_primary_mut() {
            match window.mode() {
                bevy::window::WindowMode::Windowed => {
                    window.set_mode(bevy::window::WindowMode::BorderlessFullscreen);
                }
                bevy::window::WindowMode::Fullscreen { .. } => {
                    window.set_mode(bevy::window::WindowMode::Windowed);
                }
                bevy::window::WindowMode::BorderlessFullscreen => {
                    window.set_mode(bevy::window::WindowMode::Windowed)
                }
                bevy::window::WindowMode::SizedFullscreen => todo!(),
            }
        }
    }
}

fn exit(key: Res<Input<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if key.just_pressed(KeyCode::Escape) {
        event_writer.send(AppExit);
    }
}
