use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::prelude::*;

pub struct SelectorCollectionPlugin;

impl Plugin for SelectorCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectorCollection>()
            .add_plugin(SelectorPlugin::<CanvasSelection>::default())
            .add_system(ui);
    }
}

#[derive(PartialEq, Clone)]
pub struct SelectorIndex {
    pub name: String,
}

#[derive(Resource, Default)]
pub struct SelectorCollection {
    pub list: Vec<SelectorIndex>,
}

fn ui(
    mut egui_context: ResMut<EguiContext>,
    collection: Res<SelectorCollection>,
    mut editor: ResMut<Editor>,
) {
    egui::Window::new("Selectors").show(egui_context.ctx_mut(), |ui| {
        for index in collection.list.iter() {
            if ui.button(index.name.to_owned()).clicked() {
                editor.add_selection(index);
            }
        }
    });
}
