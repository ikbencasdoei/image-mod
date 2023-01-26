#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(option_result_contains)]
#![feature(is_some_and)]

use std::path::Path;

use editor::Editor;
use eframe::Frame;
use egui::Context;
use file_picker::FilePicker;
use menu::menu;
use modifier::collection::process_modifiers;
use project::Project;
use view::View;

mod color;
mod editor;
mod file_picker;
mod image;
mod keybinds;
mod menu;
mod modifier;
mod project;
mod view;

#[derive(Default)]
struct App {
    project: Project,
    file_picker: FilePicker,
    view: View,
    editor: Editor,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let App {
            project,
            file_picker,
            view,
            editor,
        } = self;

        keybinds::fullscreen(ctx, frame);
        keybinds::exit(ctx, frame);

        file_picker.update(project);

        menu(ctx, view, project, file_picker);

        project.view(ctx, editor);

        process_modifiers(project, ctx, view, editor);

        if project.output_changed() {
            if let Some(output) = project.get_output() {
                view.update(ctx, output);
            }
        }

        view.process(ctx);
    }
}

fn main() {
    dotenvy::dotenv().ok();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        centered: true,
        ..Default::default()
    };

    let project = if let Ok(path) = std::env::var("NEW_PROJECT_INPUT_PATH") {
        Project::new_from_input_path(Path::new(&path))
    } else {
        Project::default()
    };

    eframe::run_native(
        env!("CARGO_PKG_NAME"),
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.5);

            let mut editor = Editor::default();
            modifier::collection::init_modifiers_collection(&mut editor);

            Box::new(App {
                project,
                editor,
                ..Default::default()
            })
        }),
    );
}
