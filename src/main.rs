#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(option_result_contains)]
#![feature(is_some_and)]

use std::path::Path;

use bevy::prelude::{App as BevyApp, Color, *};
use bevy_egui::{EguiPlugin, EguiSettings};
use editor::{Editor, EditorPlugin};
use eframe::Frame;
use egui::Context;
use file_picker::FilePickerPlugin;
use keybinds::KeyBindsPlugin;
use menu::MenuPlugin;
use mods::ui::ModifierUiPlugin;
use view::ViewPlugin;

mod color;
mod editor;
mod file_picker;
mod image;
mod keybinds;
mod menu;
mod mods;
mod view;

#[derive(Default)]
struct App {}

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
    eframe::run_native(
        env!("CARGO_PKG_NAME"),
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.5);
            Box::new(App::default())
        }),
    );

    //

    // BevyApp::new()
    //     .insert_resource(Msaa { samples: 4 })
    //     .insert_resource(ClearColor(Color::DARK_GRAY))

    //     .add_plugin(EditorPlugin)
    //     .add_plugin(FilePickerPlugin)
    //     .add_plugin(ViewPlugin)
    //     .add_plugin(MenuPlugin)
    //     .add_plugin(KeyBindsPlugin)
    //     .add_plugin(ModifierUiPlugin)
    //     .add_startup_system(setup)
    //     .run();
}

fn setup(mut egui_settings: ResMut<EguiSettings>, mut editor: ResMut<Editor>) {
    // egui_settings.scale_factor = 1.5;

    if let Ok(path) = std::env::var("NEW_PROJECT_INPUT_PATH") {
        *editor = Editor::new_from_input_path(Path::new(&path))
    }
}
