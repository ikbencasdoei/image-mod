use std::{
    path::{Path, PathBuf},
    str::FromStr,
    sync::mpsc::Receiver,
};

use egui::{TextEdit, Ui};

use crate::{
    editor::Editor,
    file_picker::PickerResult,
    image::Image,
    modifier::{modification::Output, traits::Modifier},
};

#[derive(Default)]
pub struct Source {
    pub path: PathBuf,
    receiver: Option<Receiver<Option<PickerResult>>>,
}

impl Source {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            ..Default::default()
        }
    }
}

impl Clone for Source {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            receiver: None,
        }
    }
}

impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Modifier for Source {
    fn apply(&mut self, _: Output) -> Option<Image> {
        Image::open(&self.path).ok()
    }

    fn view(&mut self, ui: &mut Ui, editor: &mut Editor) {
        if let Some(receiver) = &self.receiver {
            if let Ok(Some(PickerResult::PickedLoad(result))) = receiver.try_recv() {
                self.path = result;
            }
        }

        ui.horizontal(|ui| {
            ui.label("path:");
            let mut string = self.path.to_string_lossy().to_string();
            if ui.add(TextEdit::singleline(&mut string)).changed() {
                self.path = PathBuf::from_str(&string).unwrap();
            }
        });

        ui.add_enabled_ui(!editor.picker.is_open(), |ui| {
            if ui.button("open file picker").clicked() {
                self.receiver = editor.picker.picker_with_channel().ok();
            }
        });
    }
}
