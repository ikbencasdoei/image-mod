use std::{
    path::PathBuf,
    sync::mpsc::{channel, Receiver},
    thread::{self, JoinHandle},
};

use rfd::FileDialog;

use crate::project::Project;

#[derive(Default)]
pub struct FilePicker {
    pub open: Option<JoinHandle<Option<PickerResult>>>,
}

#[derive(Clone)]
pub enum PickerResult {
    PickedLoad(PathBuf),
    PickedExport(PathBuf),
}

impl FilePicker {
    pub fn picker_with_channel(&self) -> Result<Receiver<Option<PickerResult>>, &str> {
        if self.open.is_some() {
            return Err("file picker already open");
        }

        let dialog = FileDialog::new().add_filter("image", &["png", "jpg"]);
        let (sender, receiver) = channel();
        thread::spawn(move || {
            let result = dialog.pick_file().map(PickerResult::PickedLoad);
            sender.send(result.clone()).ok();
        });

        Ok(receiver)
    }

    pub fn open_load(&mut self) -> Result<(), &str> {
        let dialog = FileDialog::new().add_filter("image", &["png", "jpg"]);

        self.spawn(|| dialog.pick_file().map(PickerResult::PickedLoad))?;

        Ok(())
    }

    pub fn open_export(
        &mut self,
        directory: impl Into<PathBuf>,
        file_name: String,
    ) -> Result<(), &str> {
        let directory: PathBuf = directory.into();
        let dialog = FileDialog::new()
            .add_filter("png", &["png"])
            .add_filter("jpg", &["jpg"])
            .set_directory(directory)
            .set_file_name(&file_name);

        self.spawn(move || dialog.save_file().map(PickerResult::PickedExport))?;

        Ok(())
    }

    fn spawn(
        &mut self,
        closure: impl FnOnce() -> Option<PickerResult> + Send + 'static,
    ) -> Result<(), &str> {
        if self.open.is_some() {
            return Err("file picker already open");
        }

        self.open = Some(thread::spawn(closure));

        Ok(())
    }

    pub fn update(&mut self, project: &mut Project) {
        if self.open.is_some() && self.open.as_ref().unwrap().is_finished() {
            if let Ok(Some(event)) = self.open.take().unwrap().join() {
                match event {
                    PickerResult::PickedLoad(path) => *project = Project::new_from_input_path(path),
                    PickerResult::PickedExport(path) => project.export(path).unwrap(),
                }
            }
        }
    }
}
