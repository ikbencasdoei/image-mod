use std::path::PathBuf;

use futures_lite::future;

use bevy::{
    prelude::*,
    tasks::{IoTaskPool, Task},
};
use bevy_egui::{egui, EguiContext};
use rfd::AsyncFileDialog;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_event::<Event>()
            .add_system(ui)
            .add_system(events)
            .add_system(task);
    }
}

#[derive(Component)]
pub struct State {
    picker_open: bool,
    task: Option<Task<Event>>,
}

pub enum Event {
    PickerOpened,
    PickedOpen(PathBuf),
    PickedSave(PathBuf),
    NothingPicked,
}

fn setup(mut commands: Commands) {
    commands.spawn(State {
        picker_open: false,
        task: None,
    });
}

fn events(mut state: Query<&mut State>, mut event_reader: EventReader<Event>) {
    for mut i in state.iter_mut() {
        for event in event_reader.iter() {
            match event {
                Event::PickerOpened => i.picker_open = true,
                Event::PickedOpen(_) | Event::PickedSave(_) | Event::NothingPicked => {
                    i.picker_open = false;
                }
            }
        }
    }
}

fn task(mut state: Query<&mut State>, mut event_writer: EventWriter<Event>) {
    for mut i in state.iter_mut() {
        let mut finished = false;
        if let Some(task) = &mut i.task {
            if let Some(result) = future::block_on(future::poll_once(task)) {
                finished = true;
                event_writer.send(result);
            }
        }

        if finished {
            i.task = None;
        }
    }
}

fn ui(
    mut query: Query<&mut State>,
    mut egui_context: ResMut<EguiContext>,
    mut event_writer: EventWriter<Event>,
    mut query_sprite: Query<&mut crate::viewer::Sprite>,
) {
    let pool = IoTaskPool::get();

    for mut state in query.iter_mut() {
        egui::TopBottomPanel::top("panel").show(egui_context.ctx_mut(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.add_enabled_ui(!state.picker_open, |ui| {
                    load_button(&mut state, ui, &mut event_writer, pool, &mut query_sprite);
                });

                ui.add_enabled_ui(
                    {
                        if let Ok(sprite) = query_sprite.get_single() {
                            sprite.image_path.is_some()
                        } else {
                            false
                        }
                    } && !state.picker_open,
                    |ui| {
                        save_button(&mut state, ui, &mut event_writer, pool, &mut query_sprite);
                    },
                );

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
                    if let Ok(sprite) = query_sprite.get_single() {
                        if let Some(image_path) = sprite.image_path.as_ref() {
                            ui.label(image_path.to_string_lossy());
                        } else {
                            ui.label(NO_IMAGE_TEXT);
                        }
                    } else {
                        ui.label(NO_IMAGE_TEXT);
                    }
                }
            });
        });
    }
}

fn load_button(
    state: &mut Mut<State>,
    ui: &mut egui::Ui,
    event_writer: &mut EventWriter<Event>,
    pool: &IoTaskPool,
    query_sprite: &mut Query<&mut crate::viewer::Sprite>,
) {
    if ui.button("load").clicked() {
        let directory = if let Ok(sprite) = query_sprite.get_single() {
            if let Some(path) = sprite.image_path.clone() {
                path
            } else {
                PathBuf::new()
            }
        } else {
            PathBuf::new()
        };

        event_writer.send(Event::PickerOpened);
        let future = async move {
            match AsyncFileDialog::new()
                .add_filter("image", &["png", "jpg"])
                .set_directory(directory)
                .pick_file()
                .await
            {
                Some(file) => Event::PickedOpen(PathBuf::from(file)),
                None => Event::NothingPicked,
            }
        };

        state.task = Some(pool.spawn(future));
    }
}

fn save_button(
    state: &mut Mut<State>,
    ui: &mut egui::Ui,
    event_writer: &mut EventWriter<Event>,
    pool: &IoTaskPool,
    query_sprite: &mut Query<&mut crate::viewer::Sprite>,
) {
    if ui.button("save").clicked() {
        event_writer.send(Event::PickerOpened);
        let directory = if let Ok(sprite) = query_sprite.get_single() {
            if let Some(path) = sprite.image_path.clone() {
                path
            } else {
                PathBuf::new()
            }
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
                Some(file) => Event::PickedSave(PathBuf::from(file)),
                None => Event::NothingPicked,
            }
        };

        state.task = Some(pool.spawn(future));
    }
}
