use bevy::prelude::*;

use bevy_egui::{egui, EguiContext};

use super::{
    pencils::{simple::SimplePencil, PencilCollection},
    plugin::{Tool, ToolIndex},
};

pub struct ToolBoxPlugin;
impl Plugin for ToolBoxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ToolCollection::default())
            .insert_resource(CurrentTool::default())
            .add_plugin(PencilCollection)
            .add_system(ui)
            .add_system(sort);
    }
}

#[derive(Resource, Default)]
pub struct ToolCollection {
    pub tools: Vec<ToolIndex>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct CurrentTool(Option<ToolIndex>);

impl Default for CurrentTool {
    fn default() -> Self {
        Self(Some(SimplePencil::get_index()))
    }
}

fn sort(mut tool_collection: ResMut<ToolCollection>) {
    if tool_collection.is_changed() {
        tool_collection
            .tools
            .sort_by(|a, b| a.description.name.partial_cmp(&b.description.name).unwrap());
    }
}

fn ui(
    mut current_tool: ResMut<CurrentTool>,
    mut egui_context: ResMut<EguiContext>,
    tool_collection: Res<ToolCollection>,
) {
    egui::Window::new("Tools").show(egui_context.ctx_mut(), |ui| {
        for tool in tool_collection.tools.iter() {
            if ui
                .radio(
                    **current_tool == Some(tool.to_owned()),
                    tool.description.name.to_string(),
                )
                .clicked()
            {
                **current_tool = Some(tool.to_owned());
            };
        }
    });
}
