use std::{
    path::PathBuf,
    thread::{self, JoinHandle},
};

use rfd::FileDialog;

use crate::editor::Editor;

#[derive(Default)]
pub struct FilePicker {
    pub open: Option<JoinHandle<Option<FilePickerEvent>>>,
}

pub enum FilePickerEvent {
    PickedLoad(PathBuf),
    PickedExport(PathBuf),
}

impl FilePicker {
    pub fn open_load(&mut self) -> Result<(), &str> {
        let dialog = FileDialog::new().add_filter("image", &["png", "jpg"]);

        self.spawn(|| dialog.pick_file().map(FilePickerEvent::PickedLoad))?;

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

        self.spawn(move || dialog.save_file().map(FilePickerEvent::PickedExport))?;

        Ok(())
    }

    fn spawn(
        &mut self,
        closure: impl FnOnce() -> Option<FilePickerEvent> + Send + 'static,
    ) -> Result<(), &str> {
        if self.open.is_some() {
            return Err("file picker already open");
        }

        self.open = Some(thread::spawn(closure));

        Ok(())
    }

    pub fn update(&mut self, editor: &mut Editor) {
        if self.open.is_some() && self.open.as_ref().unwrap().is_finished() {
            if let Ok(Some(event)) = self.open.take().unwrap().join() {
                match event {
                    FilePickerEvent::PickedLoad(path) => {
                        *editor = Editor::new_from_input_path(path)
                    }
                    FilePickerEvent::PickedExport(path) => editor.export(path).unwrap(),
                }
            }
        }
    }
}
