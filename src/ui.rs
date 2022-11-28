use std::path::PathBuf;

use futures_lite::future;

use bevy::{
    prelude::*,
    tasks::{IoTaskPool, Task},
};
use bevy_egui::{egui, EguiContext};
use rfd::AsyncFileDialog;

use crate::project::Project;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<State>()
            .add_event::<FilePickerEvent>()
            .add_system(ui)
            .add_system(events)
            .add_system(task);
    }
}

#[derive(Resource)]
pub struct State {
    task: Option<Task<FilePickerEvent>>,
}

impl Default for State {
    fn default() -> Self {
        Self { task: None }
    }
}

pub enum FilePickerEvent {
    PickerOpened,
    PickedOpen(PathBuf),
    PickedExport(PathBuf),
    NothingPicked,
}

fn events(mut event_reader: EventReader<FilePickerEvent>, mut project: ResMut<Project>) {
    for event in event_reader.iter() {
        match event {
            FilePickerEvent::PickerOpened => (),
            FilePickerEvent::PickedOpen(path) => {
                *project = Project::new_from_input_path(path).unwrap()
            }
            FilePickerEvent::PickedExport(path) => {
                project.export(path).unwrap();
            }
            _ => (),
        }
    }
}

fn task(mut state: ResMut<State>, mut event_writer: EventWriter<FilePickerEvent>) {
    let mut finished = false;
    if let Some(task) = &mut state.task {
        if let Some(result) = future::block_on(future::poll_once(task)) {
            finished = true;
            event_writer.send(result);
        }
    }

    if finished {
        state.task = None;
    }
}

fn ui(
    mut state: ResMut<State>,
    mut egui_context: ResMut<EguiContext>,
    mut event_writer: EventWriter<FilePickerEvent>,
    mut query_sprite: Query<&mut crate::view::View>,
    project: Res<Project>,
) {
    let pool = IoTaskPool::get();

    egui::TopBottomPanel::top("panel").show(egui_context.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.add_enabled_ui(state.task.is_none(), |ui| {
                load_button(&mut state, ui, &mut event_writer, pool, &project);
            });

            ui.add_enabled_ui(project.path.is_some() && state.task.is_none(), |ui| {
                save_button(&mut state, ui, &mut event_writer, pool, &project);
            });

            if let Ok(sprite) = &mut query_sprite.get_single_mut() {
                if let Some(scale) = &mut sprite.target_scale {
                    ui.separator();

                    let mut single = scale.x * 100.0;

                    ui.add(
                        egui::DragValue::new(&mut single)
                            .clamp_range(1.0..=f32::MAX)
                            .suffix("%")
                            .speed(1),
                    );

                    single /= 100.0;
                    *scale = Vec3::new(single, single, single);
                }
            }

            ui.separator();

            {
                const NO_IMAGE_TEXT: &str = "(no image)";
                if let Some(image_path) = project.path.as_ref() {
                    ui.label(image_path.to_string_lossy());
                } else {
                    ui.label(NO_IMAGE_TEXT);
                }
            }
        });
    });
}

fn load_button(
    state: &mut ResMut<State>,
    ui: &mut egui::Ui,
    event_writer: &mut EventWriter<FilePickerEvent>,
    pool: &IoTaskPool,
    project: &Res<Project>,
) {
    if ui.button("load").clicked() {
        let directory = if let Some(path) = project.path.clone() {
            path
        } else {
            PathBuf::new()
        };

        event_writer.send(FilePickerEvent::PickerOpened);
        let future = async move {
            match AsyncFileDialog::new()
                .add_filter("image", &["png", "jpg"])
                .set_directory(directory)
                .pick_file()
                .await
            {
                Some(file) => FilePickerEvent::PickedOpen(PathBuf::from(file)),
                None => FilePickerEvent::NothingPicked,
            }
        };

        state.task = Some(pool.spawn(future));
    }
}

fn save_button(
    state: &mut ResMut<State>,
    ui: &mut egui::Ui,
    event_writer: &mut EventWriter<FilePickerEvent>,
    pool: &IoTaskPool,
    project: &Res<Project>,
) {
    if ui.button("export").clicked() {
        event_writer.send(FilePickerEvent::PickerOpened);
        let directory = if let Some(path) = project.path.clone() {
            path
        } else {
            PathBuf::new()
        };

        let file_name = directory
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let future = async move {
            match AsyncFileDialog::new()
                .add_filter("png", &["png"])
                .add_filter("jpg", &["jpg"])
                .set_directory(directory)
                .set_file_name(file_name.as_str())
                .save_file()
                .await
            {
                Some(file) => FilePickerEvent::PickedExport(PathBuf::from(file)),
                None => FilePickerEvent::NothingPicked,
            }
        };

        state.task = Some(pool.spawn(future));
    }
}
