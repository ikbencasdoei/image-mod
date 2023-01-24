use std::path::PathBuf;

use crate::{
    editor::Editor,
    file_picker::{FilePicker, FilePickerEvent},
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(menu.label(MenuSystemLabel));
    }
}

fn menu(
    mut egui_context: ResMut<EguiContext>,
    mut query_sprite: Query<&mut crate::view::View>,
    editor: Res<Editor>,
    mut file_picker: ResMut<FilePicker>,
) {
    egui::TopBottomPanel::top("panel").show(egui_context.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.add_enabled_ui(file_picker.open.is_none(), |ui| {
                if ui.button("new").clicked() {
                    file_picker.open_load().ok();
                }
            });

            ui.add_enabled_ui(
                editor.get_path().is_some() && file_picker.open.is_none(),
                |ui| {
                    if ui.button("export").clicked() {
                        let directory = if let Some(path) = editor.get_path() {
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
                },
            );

            if let Ok(view) = query_sprite.get_single_mut().as_deref_mut() {
                if view.target_scale.is_some() {
                    ui.separator();

                    let mut percentage = view.target_scale.unwrap().x * 100.0;

                    let response = ui.add(
                        egui::DragValue::new(&mut percentage)
                            .clamp_range(1.0..=f32::MAX)
                            .suffix("%")
                            .speed(1),
                    );

                    if response.secondary_clicked() {
                        percentage = 100.0;
                        if let Some(translation) = &mut view.target_translation {
                            *translation = Vec3::ZERO;
                        }
                    }

                    if response.changed() {
                        if let Some(translation) = &mut view.target_translation {
                            *translation = Vec3::ZERO;
                        }
                    }

                    percentage /= 100.0;
                    *view.target_scale.as_mut().unwrap() =
                        Vec3::new(percentage, percentage, percentage);
                }
            }

            ui.separator();

            {
                if let Some(image_path) = editor.get_path() {
                    ui.label(image_path.to_string_lossy());
                } else {
                    ui.label("(no image)");
                }
            }
        });
    });
}
