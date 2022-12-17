use std::any::TypeId;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use dyn_clone::DynClone;

use crate::prelude::*;

pub struct SelectorCollectionPlugin;

impl Plugin for SelectorCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectorCollection>()
            .add_plugin(SelectorPlugin::<CanvasSelection>::default())
            .add_system(ui);
    }
}

pub trait SelInstancer: Fn() -> Box<dyn Selector + Send + Sync + 'static> + DynClone {
    fn instance(&self) -> Box<dyn Selector + Send + Sync>;
}

impl<T: Fn() -> Box<dyn Selector + Send + Sync + 'static> + DynClone> SelInstancer for T {
    fn instance(&self) -> Box<dyn Selector + Send + Sync> {
        self()
    }
}

dyn_clone::clone_trait_object!(SelInstancer);

#[derive(Clone)]
pub struct SelectorIndex {
    pub name: String,
    pub id: TypeId,
    pub instancer: Box<dyn SelInstancer + Send + Sync + 'static>,
}

impl PartialEq for SelectorIndex {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
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
    egui::Window::new("Add selection").show(egui_context.ctx_mut(), |ui| {
        ui.add_enabled_ui(editor.get_selected_mod().is_some(), |ui| {
            for index in collection.list.iter() {
                if ui
                    .button(index.name.to_owned())
                    .on_disabled_hover_text("First select an modifier")
                    .clicked()
                {
                    editor.add_selection(index);
                }
            }
        });
    });
}
