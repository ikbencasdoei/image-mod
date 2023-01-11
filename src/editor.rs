use std::{
    path::{Path, PathBuf},
    slice::{Iter, IterMut},
};

use bevy::prelude::*;
use uuid::Uuid;

use crate::{
    image::Image,
    mods::{
        collection::{source::Source, ModifierIndex},
        modifier::Modification,
        plugin::Modifier,
    },
};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Editor>();
    }
}

#[derive(Resource, Default)]
pub struct Editor {
    mods: Vec<Modification>,
    selected_mod: Option<Uuid>,
}

impl Editor {
    pub fn new_from_input_path(path: impl AsRef<Path>) -> Self {
        Self {
            mods: vec![Modification::new(Source::new(path))],
            ..default()
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
        let mut reversed: Vec<&mut Modification> = self.mods.iter_mut().rev().collect();

        let (modification, inputs) = if !reversed.is_empty() {
            reversed.split_at_mut(1)
        } else {
            (reversed.as_mut_slice(), &mut [] as &mut [&mut Modification])
        };

        if let Some(modification) = modification.get_mut(0) {
            modification.get_output(inputs).image.clone()
        } else {
            None
        }
    }

    pub fn add_mod(&mut self, index: &ModifierIndex) {
        let new = Modification::new_from_index(index.clone());
        self.selected_mod = Some(new.id);
        self.mods.push(new);
    }

    fn get_mod_mut(&mut self, id: Uuid) -> Option<&mut Modification> {
        self.mods.iter_mut().find(|item| item.id == id)
    }

    #[allow(dead_code)]
    fn get_mod(&self, id: Uuid) -> Option<&Modification> {
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

    pub fn iter_mods(&self) -> Iter<Modification> {
        self.mods.iter()
    }

    pub fn iter_mut_mods(&mut self) -> IterMut<Modification> {
        self.mods.iter_mut()
    }

    pub fn get_mods(&self) -> &Vec<Modification> {
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

    pub fn get_selected_mod_mut(&mut self) -> Option<&mut Modification> {
        self.selected_mod.and_then(|id| self.get_mod_mut(id))
    }

    #[allow(dead_code)]
    pub fn get_selected_mod(&self) -> Option<&Modification> {
        self.selected_mod.and_then(|id| self.get_mod(id))
    }

    pub fn get_selected_mod_id(&self) -> Option<Uuid> {
        self.selected_mod
    }

    #[allow(dead_code)]
    pub fn get_when_selected<T: Modifier + Default + Send + Sync + 'static>(&self) -> Option<&T> {
        self.get_selected_mod()
            .and_then(|modification| modification.get_modifier())
    }

    pub fn get_when_selected_mut<T: Modifier + Default + Send + Sync + 'static>(
        &mut self,
    ) -> Option<&mut T> {
        self.get_selected_mod_mut()
            .and_then(|modification| modification.get_modifier_mut())
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

    fn get_mods_of_type<T: Modifier + Default + Send + Sync + 'static>(&self) -> Vec<&T> {
        self.iter_mods()
            .map(|modification| modification.get_modifier())
            .flatten()
            .collect()
    }

    pub fn get_path(&self) -> Option<PathBuf> {
        self.get_mods_of_type::<Source>()
            .last()
            .map(|source| source.path.clone())
    }
}
