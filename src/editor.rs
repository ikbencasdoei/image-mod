use std::{
    path::{Path, PathBuf},
    slice::IterMut,
};

use bevy::prelude::*;
use image::ImageError;

use crate::prelude::{Image, *};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Editor>();
    }
}

#[derive(Resource, Default)]
pub struct Editor {
    pub input: Image,
    pub path: Option<PathBuf>,
    mods: Vec<Modification>,
    pub add_mod_index: Option<ModifierIndex>,
    pub add_sel_index: Option<SelectorIndex>,
    pub selected_mod: Option<usize>,
}

impl Editor {
    pub fn new_from_input_path(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        Ok(Self {
            input: Image::open(path.as_ref())?,
            path: Some(path.as_ref().to_path_buf()),
            ..default()
        })
    }

    pub fn export(&mut self, path: impl AsRef<Path>) -> Result<(), ImageError> {
        self.get_output().save(path)
    }

    pub fn get_output(&mut self) -> Image {
        let mut output = self.input.clone();

        for modifier in &mut self.mods {
            modifier.apply(&mut output);
        }

        output
    }

    pub fn receive_mod(
        &mut self,
        index: ModifierIndex,
        modifier: impl Modifier + Default + Send + Sync + 'static,
    ) {
        if Some(index) == self.add_mod_index.take() {
            let mut new = Modification::new(modifier);
            new.add_selection(CanvasSelection);
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
            if let Some(selected) = self.selected_mod {
                if let Some(modifier) = self.mods.get_mut(selected) {
                    modifier.add_selection(selection);
                }
            }
        } else {
            panic!("diffrent selector received")
        }
    }

    pub fn add_selection(&mut self, index: &SelectorIndex) {
        self.add_sel_index = Some(index.clone());
    }

    pub fn remove_mod(&mut self, index: usize) {
        self.mods.remove(index);

        if let Some(selected) = self.selected_mod {
            if selected == index {
                self.selected_mod = None;
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
