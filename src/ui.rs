use std::path::PathBuf;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ui).add_system(events);
    }
}

fn events(mut event_reader: EventReader<FilePickerEvent>, mut editor: ResMut<Editor>) {
    for event in event_reader.iter() {
        match event {
            FilePickerEvent::PickedLoad(path) => *editor = Editor::new_from_input_path(path),
            FilePickerEvent::PickedExport(path) => editor.export(path).unwrap(),
        }
    }
}

fn ui(
    mut egui_context: ResMut<EguiContext>,
    mut query_sprite: Query<&mut crate::view::View>,
    editor: Res<Editor>,
    mut file_picker: ResMut<FilePicker>,
) {
    egui::TopBottomPanel::top("panel").show(egui_context.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.add_enabled_ui(file_picker.open.is_none(), |ui| {
                if ui.button("load").clicked() {
                    file_picker.open_load().ok();
                }
            });

            ui.add_enabled_ui(editor.path.is_some() && file_picker.open.is_none(), |ui| {
                if ui.button("export").clicked() {
                    let directory = if let Some(path) = editor.path.clone() {
                        path
                    } else {
                        PathBuf::new()
                    };

                    let file_name = directory
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    file_picker.open_export(directory, file_name).ok();
                }
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
                if let Some(image_path) = editor.path.as_ref() {
                    ui.label(image_path.to_string_lossy());
                } else {
                    ui.label("(no image)");
                }
            }
        });
    });
}
