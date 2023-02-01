use std::path::{Path, PathBuf};

use crate::{
    image::Image,
    modifier::{
        collection::{list::List, source::Source},
        modification::{Cation, Output},
    },
};

pub struct Project {
    pub root: Cation<List>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            root: Cation::new(List::default()),
        }
    }
}

impl Project {
    pub fn new_from_input_path(path: impl AsRef<Path>) -> Self {
        Self {
            root: Cation::new(List::from_vec_mods(vec![Source::new(path)])),
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

    pub fn get_output(&mut self) -> &Option<Image> {
        let input = Output::new_empty();
        &self.root.get_output(&input).image
    }

    pub fn output_changed(&self) -> bool {
        !self.root.check_cache(&Output::new_empty())
    }

    pub fn get_path(&self) -> Option<PathBuf> {
        self.root
            .modifier
            .get_mods_of_type::<Source>()
            .last()
            .map(|source| source.path.clone())
    }
}
