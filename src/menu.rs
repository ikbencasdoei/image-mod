use std::path::PathBuf;

use egui::{Context, Vec2};

use crate::{file_picker::FilePicker, project::Project, view::View};

pub fn menu(ctx: &Context, view: &mut View, project: &Project, file_picker: &mut FilePicker) {
    egui::TopBottomPanel::top("panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.add_enabled_ui(file_picker.is_open(), |ui| {
                if ui.button("new").clicked() {
                    file_picker.open_load().ok();
                }
            });

            ui.add_enabled_ui(project.path().is_some() && file_picker.is_open(), |ui| {
                if ui.button("export").clicked() {
                    let directory = if let Some(path) = project.path() {
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

            ui.separator();

            let mut percentage = view.scale * 100.0;

            let response = ui.add(
                egui::DragValue::new(&mut percentage)
                    .clamp_range(1.0..=f32::MAX)
                    .suffix("%")
                    .speed(1),
            );

            if response.secondary_clicked() {
                percentage = 100.0;

                view.translation = Vec2::ZERO;
            }

            if response.changed() {
                view.translation = Vec2::ZERO;
            }

            view.scale = percentage / 100.0;

            ui.separator();

            {
                if let Some(image_path) = project.path() {
                    ui.label(image_path.to_string_lossy());
                } else {
                    ui.label("(no image)");
                }
            }
        });
    });
}
