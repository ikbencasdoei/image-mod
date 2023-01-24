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
use modifier::{collection::process_modifiers, ui::ModifierUi};
use view::View;

mod color;
mod editor;
mod file_picker;
mod image;
mod keybinds;
mod menu;
mod modifier;
mod view;

#[derive(Default)]
struct App {
    editor: Editor,
    file_picker: FilePicker,
    view: View,
    mod_ui: ModifierUi,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let App {
            editor,
            file_picker,
            view,
            mod_ui,
        } = self;

        keybinds::fullscreen(ctx, frame);
        keybinds::exit(ctx, frame);

        file_picker.update(editor);

        menu(ctx, view, editor, file_picker);
        mod_ui.view(editor, ctx);

        process_modifiers(editor, ctx, view);

        if let Some(output) = editor.get_output() {
            view.update(ctx, output);
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

            let mut mod_ui = ModifierUi::default();
            modifier::collection::init_modifiers_collection(&mut mod_ui);

            Box::new(App {
                editor,
                mod_ui,
                ..Default::default()
            })
        }),
    );
}
