use bevy::prelude::{Image as BevyImage, *};

use crate::{image::Image, project::Project, ui::FilePickerEvent};

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(events)
            .add_system(update);
    }
}

#[derive(Component, Default)]
pub struct View {
    pub target_scale: Option<Vec3>,
    pub target_translation: Option<Vec3>,
}

fn setup(mut commands: Commands, mut assets: ResMut<Assets<BevyImage>>) {
    commands.spawn(Camera2dBundle::default());

    let image = Image::default();
    let handle = assets.add(image.into_bevy_image());

    commands
        .spawn(SpriteBundle {
            texture: handle,
            ..default()
        })
        .insert(View::default());
}

fn update(
    handles: Query<&Handle<BevyImage>, With<View>>,
    mut assets: ResMut<Assets<BevyImage>>,
    project: Res<Project>,
) {
    for handle in handles.iter() {
        let image = assets.get_mut(handle).unwrap();
        *image = project.get_output().into_bevy_image();
    }
}

fn events(mut event_reader: EventReader<FilePickerEvent>, mut project: ResMut<Project>) {
    for event in event_reader.iter() {
        match event {
            FilePickerEvent::PickerOpened => (),
            FilePickerEvent::PickedOpen(path) => {
                *project = Project::new_from_input_path(path).unwrap()
            }
            FilePickerEvent::PickedSave(path) => {
                let image = project.get_output();
                image.save(path).unwrap();
            }
            _ => (),
        }
    }
}
