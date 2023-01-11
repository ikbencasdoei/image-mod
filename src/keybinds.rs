use bevy::{app::AppExit, prelude::*};

pub struct KeyBindsPlugin;

impl Plugin for KeyBindsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fullscreen).add_system(exit);
    }
}

fn fullscreen(mut windows: ResMut<Windows>, key: Res<Input<KeyCode>>) {
    if key.just_pressed(KeyCode::F11) {
        if let Some(window) = windows.get_primary_mut() {
            match window.mode() {
                WindowMode::Windowed => {
                    window.set_mode(WindowMode::BorderlessFullscreen);
                }
                WindowMode::Fullscreen { .. } => {
                    window.set_mode(WindowMode::Windowed);
                }
                WindowMode::BorderlessFullscreen => window.set_mode(WindowMode::Windowed),
                WindowMode::SizedFullscreen => todo!(),
            }
        }
    }
}

fn exit(key: Res<Input<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if key.just_pressed(KeyCode::Escape) {
        event_writer.send(AppExit);
    }
}
