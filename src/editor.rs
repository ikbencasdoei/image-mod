use egui::Context;
use uuid::Uuid;

use crate::{
    modifier::{
        modification::{Cacher, DynMod},
        traits::{Modifier, ModifierIndex},
    },
    project::Project,
};

#[derive(Default)]
pub struct Editor {
    pub index: Vec<ModifierIndex>,
    pub selected: Option<Uuid>,
    pub dragging: Option<Cacher<DynMod>>,
    pub dropped: Option<Cacher<DynMod>>,
}

impl Editor {
    pub fn add_index(&mut self, index: ModifierIndex) {
        self.index.push(index);
        self.index.sort_by(|a, b| a.name.cmp(&b.name));
    }

    pub fn view(&mut self, ctx: &Context, project: &mut Project) {
        egui::SidePanel::left("Modifiers")
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(format!(
                        "Modifiers ({})",
                        project.root.modifier.contents.len()
                    ));
                });
                ui.separator();
                project.root.modifier.view(ui, self);
            });

        if !ctx.memory().is_anything_being_dragged() {
            if self.dragging.is_some() {
                self.dropped = self.dragging.take();
                ctx.request_repaint();
            }
        }
    }
}
