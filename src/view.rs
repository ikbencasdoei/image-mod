use bevy::{
    prelude::*,
    render::{render_resource::SamplerDescriptor, texture::ImageSampler},
};
use image::{DynamicImage, RgbImage};

use crate::{project::Project, ui::FilePickerEvent};

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

fn setup(mut commands: Commands, mut assets: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());

    let data = DynamicImage::ImageRgb8(RgbImage::new(1, 1));
    let mut image = Image::from_dynamic(data, true);
    image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
        mag_filter: bevy::render::render_resource::FilterMode::Nearest,
        min_filter: bevy::render::render_resource::FilterMode::Linear,
        ..default()
    });
    let handle = assets.add(image);

    commands
        .spawn(SpriteBundle {
            texture: handle,
            ..default()
        })
        .insert(View::default());
}

fn update(
    handles: Query<&Handle<Image>, With<View>>,
    mut assets: ResMut<Assets<Image>>,
    project: Res<Project>,
) {
    for handle in handles.iter() {
        let image = assets.get_mut(handle).unwrap();
        *image = Image::from_dynamic(project.get_output(), true);
        image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
            mag_filter: bevy::render::render_resource::FilterMode::Nearest,
            min_filter: bevy::render::render_resource::FilterMode::Linear,
            ..default()
        });
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
