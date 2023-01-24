use std::{
    path::{Path, PathBuf},
    slice::{Iter, IterMut},
};

use uuid::Uuid;

use crate::{
    image::Image,
    modifier::{
        collection::{source::Source, ModifierIndex},
        modification::{DynMod, ModOutput, Modification},
        modifier::Modifier,
    },
};

#[derive(Default)]
pub struct Project {
    mods: Vec<Modification<DynMod>>,
    selected_mod: Option<Uuid>,
}

impl Project {
    pub fn new_from_input_path(path: impl AsRef<Path>) -> Self {
        Self {
            mods: vec![Modification::new(DynMod::new(Source::new(path)))],
            ..Default::default()
        }
    }

    pub fn export(&mut self, path: impl AsRef<Path>) -> Result<(), String> {
        if let Some(output) = self.get_output() {
            output.save(path).map_err(|err| err.to_string())
        } else {
            Err("no output to save".to_string())
        }
    }

    pub fn get_output(&mut self) -> Option<Image> {
        let mut output = ModOutput::new_empty();
        {
            let mut borrow = &output;
            for modification in self.mods.iter_mut() {
                borrow = modification.get_output(&borrow);
            }
            output = borrow.clone();
        }

        output.image
    }

    pub fn add_mod(&mut self, index: &ModifierIndex) {
        let new = Modification::new(DynMod::from_index(index.clone()));
        self.selected_mod = Some(new.id);
        self.mods.push(new);
    }

    pub fn get_mod_mut(&mut self, id: Uuid) -> Option<&mut Modification<DynMod>> {
        self.mods.iter_mut().find(|item| item.id == id)
    }

    pub fn get_mod(&self, id: Uuid) -> Option<&Modification<DynMod>> {
        self.mods.iter().find(|item| item.id == id)
    }

    pub fn get_mod_index(&mut self, id: Uuid) -> Option<usize> {
        self.mods
            .iter()
            .enumerate()
            .find(|item| item.1.id == id)
            .map(|item| item.0)
    }

    pub fn remove_mod(&mut self, id: Uuid) -> Result<(), &str> {
        if let Some(index) = self.get_mod_index(id) {
            self.mods.remove(index);

            if let Some(selected) = self.selected_mod {
                if selected == id {
                    self.selected_mod = None;
                }
            }
            Ok(())
        } else {
            Err("invalid id")
        }
    }

    pub fn iter_mods(&self) -> Iter<Modification<DynMod>> {
        self.mods.iter()
    }

    #[allow(dead_code)]
    pub fn iter_mut_mods(&mut self) -> IterMut<Modification<DynMod>> {
        self.mods.iter_mut()
    }

    pub fn mod_ids(&self) -> Vec<Uuid> {
        self.iter_mods().map(|modi| modi.id).collect()
    }

    pub fn get_mods(&self) -> &Vec<Modification<DynMod>> {
        &self.mods
    }

    pub fn select_mod(&mut self, id: Uuid) -> Result<(), &str> {
        if self.get_mod_index(id).is_some() {
            self.selected_mod = Some(id);
            Ok(())
        } else {
            Err("modifier doesnt exist")
        }
    }

    pub fn get_selected_mod_mut(&mut self) -> Option<&mut Modification<DynMod>> {
        self.selected_mod.and_then(|id| self.get_mod_mut(id))
    }

    #[allow(dead_code)]
    pub fn get_selected_mod(&self) -> Option<&Modification<DynMod>> {
        self.selected_mod.and_then(|id| self.get_mod(id))
    }

    pub fn get_selected_mod_id(&self) -> Option<Uuid> {
        self.selected_mod
    }

    #[allow(dead_code)]
    pub fn get_when_selected<T: Modifier + Default + 'static>(&self) -> Option<&T> {
        self.get_selected_mod()
            .and_then(|modification| modification.modifier.get_modifier())
    }

    #[allow(dead_code)]
    pub fn get_when_selected_mut<T: Modifier + Default + 'static>(&mut self) -> Option<&mut T> {
        self.get_selected_mod_mut()
            .and_then(|modification| modification.modifier.get_modifier_mut())
    }

    pub fn mod_set_index(&mut self, id: Uuid, index: usize) -> Result<(), &str> {
        if let Some(i) = self.get_mod_index(id) {
            let modification = self.mods.remove(i);
            self.mods.insert(index, modification);
            Ok(())
        } else {
            Err("invalid id")
        }
    }

    fn get_mods_of_type<T: Modifier + Default + 'static>(&self) -> Vec<&T> {
        self.iter_mods()
            .map(|modification| modification.modifier.get_modifier())
            .flatten()
            .collect()
    }

    pub fn get_path(&self) -> Option<PathBuf> {
        self.get_mods_of_type::<Source>()
            .last()
            .map(|source| source.path.clone())
    }
}
