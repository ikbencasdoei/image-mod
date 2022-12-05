use std::{
    path::{Path, PathBuf},
    slice::IterMut,
};

use bevy::prelude::*;
use image::ImageError;
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
    pub input: Option<Image>,
    pub path: Option<PathBuf>,
    mods: Vec<Modification>,
    pub add_mod_index: Option<ModifierIndex>,
    pub add_sel_index: Option<SelectorIndex>,
    selected_mod: Option<Uuid>,
}

impl Editor {
    pub fn new_from_input_path(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        Ok(Self {
            input: Some(Image::open(path.as_ref())?),
            path: Some(path.as_ref().to_path_buf()),
            ..default()
        })
    }

    pub fn export(&mut self, path: impl AsRef<Path>) -> Result<(), String> {
        if let Some(output) = self.get_output() {
            output.save(path).map_err(|err| err.to_string())
        } else {
            Err("no output to save".to_string())
        }
    }

    pub fn get_output(&mut self) -> Option<Image> {
        let mut output = &mut self.input.clone();

        for modifier in &mut self.mods {
            modifier.apply(&mut output);
        }

        output.clone()
    }

    pub fn receive_mod(
        &mut self,
        index: ModifierIndex,
        modifier: impl Modifier + Default + Send + Sync + 'static,
    ) {
        if Some(index) == self.add_mod_index.take() {
            let mut new = Modification::new(modifier);
            new.add_selection(CanvasSelection);
            self.selected_mod = Some(new.id);
            self.mods.push(new);
        } else {
            panic!("diffrent modifier received")
        }
    }

    pub fn add_mod(&mut self, index: &ModifierIndex) {
        self.add_mod_index = Some(index.clone());
    }

    pub fn receive_sel(
        &mut self,
        index: SelectorIndex,
        selection: impl Selector + Default + Send + Sync + 'static,
    ) {
        if Some(index) == self.add_sel_index.take() {
            if let Some(id) = self.selected_mod {
                if let Some(modifier) = self.get_mod(id) {
                    modifier.add_selection(selection);
                }
            }
        } else {
            panic!("diffrent selector received")
        }
    }

    fn get_mod(&mut self, id: Uuid) -> Option<&mut Modification> {
        self.mods.iter_mut().find(|item| item.id == id)
    }

    fn get_mod_index(&mut self, id: Uuid) -> Option<usize> {
        self.mods
            .iter()
            .enumerate()
            .find(|item| item.1.id == id)
            .map(|item| item.0)
    }

    pub fn add_selection(&mut self, index: &SelectorIndex) {
        self.add_sel_index = Some(index.clone());
    }

    pub fn remove_mod(&mut self, id: Uuid) {
        if let Some(index) = self.get_mod_index(id) {
            self.mods.remove(index);

            if let Some(selected) = self.selected_mod {
                if selected == id {
                    self.selected_mod = None;
                }
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

    pub fn get_selected_mod(&self) -> Option<Uuid> {
        self.selected_mod
    }
}
