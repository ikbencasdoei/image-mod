use std::{
    path::PathBuf,
    thread::{self, JoinHandle},
};

use bevy::prelude::*;
use rfd::FileDialog;

pub struct FilePickerPlugin;

impl Plugin for FilePickerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FilePicker>()
            .add_event::<FilePickerEvent>()
            .add_system(system);
    }
}

#[derive(Resource, Default)]
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
}

fn system(mut state: ResMut<FilePicker>, mut event_writer: EventWriter<FilePickerEvent>) {
    if state.open.is_some() && state.open.as_ref().unwrap().is_finished() {
        if let Ok(Some(event)) = state.open.take().unwrap().join() {
            event_writer.send(event);
        }
    }
}
