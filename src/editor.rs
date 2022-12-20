use std::{
    path::{Path, PathBuf},
    slice::IterMut,
};

use bevy::prelude::*;
use uuid::Uuid;

use crate::prelude::{Image, *};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Editor>();
    }
}

#[derive(Resource, Default)]
pub struct Editor {
    mods: Vec<Modification>,
    pub path: Option<PathBuf>,
    selected_mod: Option<Uuid>,
}

impl Editor {
    pub fn new_from_input_path(path: impl AsRef<Path>) -> Self {
        Self {
            path: Some(path.as_ref().to_path_buf()),
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

        let (modification, inputs) = if reversed.len() >= 1 {
            reversed.split_at_mut(1)
        } else {
            (reversed.as_mut_slice(), &mut [] as &mut [&mut Modification])
        };

        if let Some(modification) = modification.get_mut(0) {
            modification.get_output(inputs).get_output()
        } else {
            None
        }
    }

    pub fn insert_mod(&mut self, modifier: Modification) {
        self.selected_mod = Some(modifier.id);
        self.mods.push(modifier);
    }

    pub fn add_mod(&mut self, index: &ModifierIndex) {
        let new = Modification::new_from_index(index.clone());
        self.insert_mod(new)
    }

    fn get_mod_mut(&mut self, id: Uuid) -> Option<&mut Modification> {
        self.mods.iter_mut().find(|item| item.id == id)
    }

    fn get_mod(&self, id: Uuid) -> Option<&Modification> {
        self.mods.iter().find(|item| item.id == id)
    }

    fn get_mod_index(&mut self, id: Uuid) -> Option<usize> {
        self.mods
            .iter()
            .enumerate()
            .find(|item| item.1.id == id)
            .map(|item| item.0)
    }

    pub fn add_selection(&mut self, index: &SelectorIndex) {
        if let Some(id) = self.selected_mod {
            if let Some(modifier) = self.get_mod_mut(id) {
                modifier.add_selection_from_index(index.clone());
            }
        }
    }

    pub fn remove_mod(&mut self, id: Uuid) {
        if let Some(index) = self.get_mod_index(id) {
            self.mods.remove(index);

            if let Some(selected) = self.selected_mod {
                if selected == id {
                    self.selected_mod = None;
                }
            }

            if let Some(modification) = self.mods.get_mut(index + 1) {
                modification.cache = None;
            }
        }
    }

    pub fn iter_mut_mods(&mut self) -> IterMut<'_, Modification> {
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
        self.selected_mod.map(|id| self.get_mod_mut(id)).flatten()
    }

    pub fn get_selected_mod(&self) -> Option<&Modification> {
        self.selected_mod.map(|id| self.get_mod(id)).flatten()
    }

    pub fn get_selected_mod_id(&self) -> Option<Uuid> {
        self.selected_mod
    }

    pub fn use_mod(&mut self, index: &ModifierIndex) {
        let mut to_remove = None;
        if let Some(modification) = self.get_selected_mod_mut() {
            if modification.get_selection().is_empty() {
                to_remove = Some(modification.id)
            }
        }

        if let Some(id) = to_remove {
            self.remove_mod(id);
        }

        self.add_mod(index);
    }
}
