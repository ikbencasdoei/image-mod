use egui::Ui;
use uuid::Uuid;

use crate::{
    editor::Editor,
    image::Image,
    modifier::{
        modification::{CacheOutput, Cacher, DynMod},
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

    fn add_mod_button(&mut self, ui: &mut Ui, editor: &mut Editor) {
        ui.vertical_centered(|ui| {
            ui.menu_button("add modifier", |ui| {
                for index in editor.index.clone().iter() {
                    if ui.button(index.name.as_str()).clicked() {
                        self.add_mod_from_index(index, editor);
                        ui.close_menu();
                    }
                }
            });
        });

        ui.separator();
    }

    pub fn add_mod_from_index(&mut self, index: &ModifierIndex, editor: &mut Editor) {
        let new = Cacher::new(DynMod::from_index(index.clone()));
        editor.selected = Some(new.id);
        self.contents.push(ModifierSlot::from_cacher(new));
    }

    pub fn get_selected_mod_mut(&mut self, editor: &Editor) -> Option<&mut Cacher<DynMod>> {
        editor.selected.and_then(|id| self.get_mod_mut(id))
    }

    pub fn get_mod_mut(&mut self, id: Uuid) -> Option<&mut Cacher<DynMod>> {
        self.iter_mods_mut().find(|item| item.id == id)
    }

    pub fn iter_mods(&self) -> impl Iterator<Item = &Cacher<DynMod>> {
        self.contents.iter().flat_map(|slot| slot.get_mod())
    }

    pub fn iter_mods_mut(&mut self) -> impl Iterator<Item = &mut Cacher<DynMod>> {
        self.contents.iter_mut().flat_map(|slot| slot.get_mod_mut())
    }

    pub fn get_mods_of_type<T: Modifier + Default + 'static>(&self) -> Vec<&T> {
        self.iter_mods()
            .map(|modification| modification.modifier.get_modifier())
            .flatten()
            .collect()
    }
}

impl Modifier for List {
    fn apply(&mut self, mut output: CacheOutput) -> Option<Image> {
        {
            let mut borrow = &output;
            for modification in self.contents.iter_mut() {
                borrow = modification.get_output(borrow);
            }
            output = borrow.clone();
        }

        output.image
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
