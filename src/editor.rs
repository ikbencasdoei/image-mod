use eframe::egui::{self, Context};
use uuid::Uuid;

use crate::{
    file_picker::FilePicker,
    modifier::{
        cation::{Cation, DynMod},
        traits::{Modifier, ModifierIndex},
    },
    project::Project,
    slot::ModifierSlot,
    view::View,
};

#[derive(Default)]
pub struct Editor {
    pub index: Vec<ModifierIndex>,
    pub selected: Option<ModId>,
    pub dragging: Option<Cation<DynMod>>,
    pub dropped: Option<Cation<DynMod>>,
    pub view: View,
    pub picker: FilePicker,
    pub add_mod_text: String,
}

pub struct ModId {
    id: Uuid,
    index: ModifierIndex,
}

impl ModId {
    pub fn from_dyn_cation(cation: &Cation<DynMod>) -> Self {
        ModId {
            id: cation.id,
            index: cation.modifier.index.clone(),
        }
    }

    pub fn try_from_slot(slot: &ModifierSlot) -> Result<Self, &str> {
        if let Some(cation) = slot.mod_ref() {
            Ok(ModId {
                id: cation.id,
                index: cation.modifier.index.clone(),
            })
        } else {
            Err("slot is empty")
        }
    }
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

        if !ctx.memory(|memory| memory.is_anything_being_dragged()) {
            if self.dragging.is_some() {
                self.dropped = self.dragging.take();
                ctx.request_repaint();
            }
        }
    }

    pub fn select_cation(&mut self, cation: &Cation<DynMod>) {
        self.selected = Some(ModId::from_dyn_cation(cation));
    }

    pub fn try_select_slot<'a>(&'a mut self, slot: &'a ModifierSlot) -> Result<(), &str> {
        self.selected = Some(ModId::try_from_slot(slot)?);
        Ok(())
    }

    pub fn selected_id(&self) -> Option<Uuid> {
        self.selected.as_ref().map(|selected| selected.id)
    }

    pub fn is_modifier_selected<T: Modifier + Default + 'static>(&self) -> bool {
        self.selected
            .as_ref()
            .is_some_and(|selected| selected.index == T::index())
    }
}
