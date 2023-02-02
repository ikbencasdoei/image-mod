use std::{
    path::PathBuf,
    sync::mpsc::{channel, Receiver},
    thread::{self, JoinHandle},
};

use rfd::FileDialog;

use crate::project::Project;

#[derive(Default)]
pub struct FilePicker {
    handle: Option<JoinHandle<()>>,
    receiver: Option<Receiver<PickerResult>>,
}

#[derive(Clone)]
pub enum PickerResult {
    PickedLoad(PathBuf),
    PickedExport(PathBuf),
    Empty,
}

impl FilePicker {
    pub fn dialog_open() -> FileDialog {
        FileDialog::new().add_filter("image", &["png", "jpg"])
    }

    pub fn dialog_export(path: Option<PathBuf>) -> FileDialog {
        let mut dialog = FileDialog::new()
            .add_filter("png", &["png"])
            .add_filter("jpg", &["jpg"]);

        if let Some(path) = path {
            dialog = dialog.set_directory(&path);
            if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                dialog = dialog.set_file_name(name);
            }
        }

        dialog
    }

    pub fn picker_open(&mut self, picker: FileDialog) -> Result<Receiver<PickerResult>, &str> {
        let (sender, receiver) = channel();
        self.spawn(move || {
            let option = picker.pick_file();
            let result = if let Some(path) = option {
                PickerResult::PickedLoad(path)
            } else {
                PickerResult::Empty
            };

            sender.send(result).ok();
        })?;

        Ok(receiver)
    }

    pub fn picker_save(&mut self, picker: FileDialog) -> Result<Receiver<PickerResult>, &str> {
        let (sender, receiver) = channel();
        self.spawn(move || {
            let option = picker.save_file();
            let result = if let Some(path) = option {
                PickerResult::PickedExport(path)
            } else {
                PickerResult::Empty
            };

            sender.send(result).ok();
        })?;

        Ok(receiver)
    }

    pub fn menu_export(&mut self, path: Option<PathBuf>) -> Result<(), &str> {
        self.receiver = Some(self.picker_save(Self::dialog_export(path))?);

        Ok(())
    }

    pub fn menu_new(&mut self) -> Result<(), &str> {
        self.receiver = Some(self.picker_open(Self::dialog_open())?);

        Ok(())
    }

    fn spawn(&mut self, closure: impl FnOnce() + Send + 'static) -> Result<(), &str> {
        if self.is_open() {
            return Err("picker is already open");
        }

        self.handle = Some(thread::spawn(closure));

        Ok(())
    }

    pub fn update(&mut self, project: &mut Project) {
        if let Some(receiver) = &self.receiver {
            if let Ok(result) = receiver.try_recv() {
                match result {
                    PickerResult::PickedLoad(path) => *project = Project::new_from_input_path(path),
                    PickerResult::PickedExport(path) => project.export(path).unwrap(),
                    PickerResult::Empty => (),
                }
            }
        }

        if self
            .handle
            .as_ref()
            .is_some_and(|handle| handle.is_finished())
        {
            self.handle.take().unwrap().join().ok();
        }
    }

    pub fn is_open(&self) -> bool {
        self.handle.is_some()
    }
}
