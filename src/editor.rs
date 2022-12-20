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
    pub selected_mod: Option<ModifierIndex>,
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
        self.mods.push(modifier);
    }

    pub fn add_mod(&mut self, index: &ModifierIndex) {
        let mut new = Modification::new_from_index(index.clone());
        new.add_selection(CanvasSelection);
        self.insert_mod(new)
    }

    fn get_mod_index(&mut self, id: Uuid) -> Option<usize> {
        self.mods
            .iter()
            .enumerate()
            .find(|item| item.1.id == id)
            .map(|item| item.0)
    }

    pub fn remove_mod(&mut self, id: Uuid) {
        if let Some(index) = self.get_mod_index(id) {
            self.mods.remove(index);

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
}
