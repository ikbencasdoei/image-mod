use std::path::{Path, PathBuf};

use bevy::prelude::{Color as BevyColor, *};
use dyn_clone::DynClone;
use image::ImageError;

use crate::{color::Color, image::Image};

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
    pub mods: Vec<Modification>,
}

impl Editor {
    pub fn new_from_input_path(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        Ok(Self {
            input: Image::open(path.as_ref())?,
            path: Some(path.as_ref().to_path_buf()),
            ..default()
        })
    }

    pub fn new_test(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        let mut proj = Self::new_from_input_path(path)?;

        let mut modif = Modification::new(GrayScaleFilter);
        modif.add_selection(CanvasSelection);
        proj.mods.push(modif);

        Ok(proj)
    }

    pub fn export(&self, path: impl AsRef<Path>) -> Result<(), ImageError> {
        self.get_output().save(path)
    }

    pub fn get_output(&self) -> Image {
        let mut output = self.input.clone();

        for modifier in &self.mods {
            modifier.apply(&mut output);
        }

        output
    }
}

pub trait Modifier: DynClone {
    fn get_pixel(&mut self, position: UVec2, image: &mut Image) -> Option<Color>;
}

dyn_clone::clone_trait_object!(Modifier);

pub struct Modification {
    modifier: Box<dyn Modifier + Send + Sync>,
    selection: Vec<Box<dyn Selection + Send + Sync>>,
}

impl Modification {
    pub fn new<M>(modifier: M) -> Self
    where
        M: Modifier + Send + Sync + 'static,
    {
        Self {
            modifier: Box::new(modifier),
            selection: Vec::new(),
        }
    }

    pub fn add_selection<S>(&mut self, selection: S)
    where
        S: Selection + Send + Sync + 'static,
    {
        self.selection.push(Box::new(selection));
    }

    pub fn apply(&self, mut output: &mut Image) {
        let mut modifier_state = dyn_clone::clone_box(&self.modifier);
        for selection in self.selection.iter() {
            for position in selection.get_pixels(&output) {
                if let Some(color) = modifier_state.get_pixel(position, &mut output) {
                    output.set_pixel(position, color).unwrap();
                }
            }
        }
    }
}

#[derive(Clone)]
struct GrayScaleFilter;

impl Modifier for GrayScaleFilter {
    fn get_pixel(&mut self, position: UVec2, image: &mut Image) -> Option<Color> {
        if let Ok(pixel) = image.get_pixel(position) {
            let sum = pixel.sum() / 4.0;
            Some(Color::from(BevyColor::rgb(sum, sum, sum)))
        } else {
            None
        }
    }
}

struct CanvasSelection;

pub trait Selection {
    fn get_pixels(&self, image: &Image) -> Vec<UVec2>;
}

impl Selection for CanvasSelection {
    fn get_pixels(&self, image: &Image) -> Vec<UVec2> {
        let size = image.size();

        (0..(size.x * size.y))
            .map(|i| UVec2::new(i % size.x, i / size.x))
            .collect()
    }
}
