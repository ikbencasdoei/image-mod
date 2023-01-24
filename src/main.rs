#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(option_result_contains)]
#![feature(is_some_and)]

use std::path::Path;

use editor::Editor;
use eframe::Frame;
use egui::Context;

mod color;
mod editor;
mod file_picker;
mod image;
mod keybinds;
mod menu;
mod mods;
mod view;

#[derive(Default)]
struct App {
    editor: Editor,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {}
}

fn main() {
    dotenvy::dotenv().ok();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        centered: true,
        ..Default::default()
    };

    let editor = if let Ok(path) = std::env::var("NEW_PROJECT_INPUT_PATH") {
        Editor::new_from_input_path(Path::new(&path))
    } else {
        Editor::default()
    };

    eframe::run_native(
        env!("CARGO_PKG_NAME"),
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.5);

            Box::new(App { editor })
        }),
    );
}
