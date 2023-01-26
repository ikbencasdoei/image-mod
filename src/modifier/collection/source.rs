use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use egui::{TextEdit, Ui};

use crate::{editor::Editor, image::Image, modifier::traits::Modifier};

#[derive(Clone, Default, PartialEq)]
pub struct Source {
    pub path: PathBuf,
}

impl Source {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl Modifier for Source {
    fn apply(&mut self, _: Option<Image>) -> Option<Image> {
        Image::open(&self.path).ok()
    }

    fn view(&mut self, ui: &mut Ui, _: &mut Editor) {
        ui.horizontal(|ui| {
            ui.label("path");
            let mut string = self.path.to_string_lossy().to_string();
            let response = ui.add(TextEdit::singleline(&mut string));
            if response.changed() {
                self.path = PathBuf::from_str(&string).unwrap();
            }
        });
    }
}
