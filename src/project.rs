use std::path::{Path, PathBuf};

use egui::Context;

use crate::{
    editor::Editor,
    image::Image,
    modifier::{
        collection::{list::List, source::Source},
        modification::{DynMod, ModOutput, Modification},
        traits::Modifier,
    },
};

pub struct Project {
    pub root: Modification<List>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            root: Modification::new(List::default()),
        }
    }
}

impl Project {
    pub fn new_from_input_path(path: impl AsRef<Path>) -> Self {
        Self {
            root: Modification::new(List {
                contents: vec![Modification::new(DynMod::new(Source::new(path)))],
            }),
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
        let input = ModOutput::new_empty();
        &self.root.get_output(&input).image
    }

    pub fn output_changed(&self) -> bool {
        !self.root.check_cache(&ModOutput::new_empty())
    }

    pub fn get_path(&self) -> Option<PathBuf> {
        self.root
            .modifier
            .get_mods_of_type::<Source>()
            .last()
            .map(|source| source.path.clone())
    }

    pub fn view(&mut self, ctx: &Context, editor: &mut Editor) {
        egui::SidePanel::left("Modifiers")
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(format!("Modifiers ({})", self.root.modifier.contents.len()));
                });
                ui.separator();
                self.root.modifier.view(ui, editor);
            });
    }
}
