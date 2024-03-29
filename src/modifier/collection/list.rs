use eframe::egui::{self, Ui};
use uuid::Uuid;

use crate::{
    editor::Editor,
    modifier::{
        cation::{Cation, DynMod, Output},
        traits::{Modifier, ModifierIndex},
    },
    slot::ModifierSlot,
};

#[derive(Default, Clone, PartialEq)]
pub struct List {
    pub contents: Vec<ModifierSlot>,
}

impl List {
    pub fn from_vec_mods<T: Modifier + Default + 'static>(vec: Vec<T>) -> Self {
        Self {
            contents: vec
                .into_iter()
                .map(|modifier| ModifierSlot::from_mod(modifier))
                .collect(),
        }
    }

    pub fn add_mod_from_index(&mut self, index: &ModifierIndex, editor: &mut Editor) {
        let new = Cation::new(DynMod::from_index(index.clone()));
        editor.select_cation(&new);
        self.contents.push(ModifierSlot::from_cacher(new));
    }

    pub fn selected_mod_mut(&mut self, editor: &Editor) -> Option<&mut Cation<DynMod>> {
        editor.selected_id().and_then(|id| self.mod_mut(id))
    }

    pub fn mod_mut(&mut self, id: Uuid) -> Option<&mut Cation<DynMod>> {
        self.iter_mods_mut().find(|item| item.id == id)
    }

    pub fn iter_mods(&self) -> impl Iterator<Item = &Cation<DynMod>> {
        self.contents.iter().flat_map(|slot| slot.mod_ref())
    }

    pub fn iter_mods_mut(&mut self) -> impl Iterator<Item = &mut Cation<DynMod>> {
        self.contents.iter_mut().flat_map(|slot| slot.mod_mut())
    }

    pub fn mods_of_type<T: Modifier + Default + 'static>(&self) -> Vec<&T> {
        self.iter_mods()
            .map(|modification| modification.modifier.modifier())
            .flatten()
            .collect()
    }

    fn add_mod_button(&mut self, ui: &mut Ui, editor: &mut Editor) {
        let mut slot = ModifierSlot::Empty;
        slot.add_mod_widget(ui, editor);
        if let ModifierSlot::Modifier(_) = slot {
            editor.try_select_slot(&slot).ok();
            self.contents.push(slot);
        }
    }
}

impl Modifier for List {
    fn apply(&mut self, input: &mut Output) {
        {
            let mut borrow = &*input;
            for modification in self.contents.iter_mut() {
                borrow = modification.output(borrow);
            }
            *input = borrow.clone();
        }
    }

    fn view(&mut self, ui: &mut Ui, editor: &mut Editor) {
        self.add_mod_button(ui, editor);

        if self.contents.is_empty() {
            ui.label("(empty)");
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let current = std::mem::replace(&mut self.contents, Vec::new());
                let slots = current
                    .into_iter()
                    .enumerate()
                    .rev()
                    .enumerate()
                    .map(|(i, (from_bottom, slot))| {
                        if i > 0 {
                            vec![(from_bottom, slot), (from_bottom, ModifierSlot::default())]
                        } else {
                            vec![
                                (from_bottom, ModifierSlot::default()),
                                (from_bottom, slot),
                                (from_bottom, ModifierSlot::default()),
                            ]
                        }
                    })
                    .flatten()
                    .collect::<Vec<(usize, ModifierSlot)>>();

                let mut new = slots
                    .into_iter()
                    .map(|(i, mut slot)| {
                        slot.view(ui, editor, Some(&format!("#{i}")));

                        if slot.is_empty() {
                            None
                        } else {
                            Some(slot)
                        }
                    })
                    .flatten()
                    .collect::<Vec<ModifierSlot>>();

                new.reverse();
                self.contents = new;
            });
        }
    }
}
