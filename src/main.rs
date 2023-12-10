#![allow(incomplete_features)]
#![feature(trait_upcasting)]

use std::path::Path;

use editor::Editor;
use eframe::Frame;
use egui::Context;
use menu::menu;
use project::Project;

use crate::image::Image;

mod applied;
mod color;
mod editor;
mod file_picker;
mod image;
mod keybinds;
mod menu;
mod modifier;
mod position;
mod project;
mod slot;
mod view;

#[derive(Default)]
struct App {
    project: Project,
    editor: Editor,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let App { project, editor } = self;

        keybinds::fullscreen(ctx, frame);
        keybinds::exit(ctx, frame);

        editor.picker.update(project);

        menu(ctx, &mut editor.view, project, &mut editor.picker);

        editor.view(ctx, project);

        if project.output_changed() {
            let default = Image::default();

            editor
                .view
                .update(ctx, project.output().as_ref().unwrap_or(&default));
        }

        editor.view.process(ctx);
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
    )
    .unwrap();
}
