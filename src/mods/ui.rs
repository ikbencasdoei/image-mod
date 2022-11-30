use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::editor::Editor;

use crate::prelude::*;

pub struct ModifierCollectionPlugin;

impl Plugin for ModifierCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModifierCollection>()
            .add_plugin(ModifierPlugin::<GrayScaleFilter>::default())
            .add_system(ui);
    }
}

#[derive(PartialEq, Clone)]
pub struct ModifierIndex {
    pub name: String,
}

impl ModifierIndex {
    pub fn from_type_name(type_name: &str) -> Self {
        ModifierIndex {
            name: type_name.split("::").last().unwrap().to_string(),
        }
    }
}

#[derive(Resource, Default)]
pub struct ModifierCollection {
    pub list: Vec<ModifierIndex>,
}

fn ui(
    mut egui_context: ResMut<EguiContext>,
    collection: Res<ModifierCollection>,
    mut editor: ResMut<Editor>,
) {
    egui::Window::new("Modifiers").show(egui_context.ctx_mut(), |ui| {
        for modifier in collection.list.iter() {
            if ui
                .radio(
                    editor.selected_mod == Some(modifier.to_owned()),
                    modifier.name.to_owned(),
                )
                .clicked()
            {
                if editor.selected_mod == Some(modifier.to_owned()) {
                    editor.selected_mod = None;
                } else {
                    editor.selected_mod = Some(modifier.clone());
                }
            };
        }
    });
}
