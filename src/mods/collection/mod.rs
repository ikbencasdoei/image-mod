use std::any::TypeId;

use bevy::prelude::*;
use dyn_clone::DynClone;

use self::{
    blur::Blur,
    brighten::Brighten,
    color::ColorFilter,
    contrast::Contrast,
    grayscale::GrayScaleFilter,
    hue::Hue,
    invert::Invert,
    pencil::{rainbow::RainbowPencilPlugin, simple::SimplePencilPlugin, sort::SortPencilPlugin},
    resize::Resize,
    source::Source,
};
use super::plugin::{Modifier, ModifierPlugin};

pub mod blur;
pub mod brighten;
pub mod color;
pub mod contrast;
pub mod grayscale;
pub mod hue;
pub mod invert;
pub mod pencil;
pub mod resize;
pub mod source;

pub struct ModifierCollectionPlugin;

impl Plugin for ModifierCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModifierCollection>()
            .add_plugin(ModifierPlugin::<GrayScaleFilter>::default())
            .add_plugin(ModifierPlugin::<Source>::default())
            .add_plugin(ModifierPlugin::<ColorFilter>::default())
            .add_plugin(ModifierPlugin::<Hue>::default())
            .add_plugin(ModifierPlugin::<Brighten>::default())
            .add_plugin(ModifierPlugin::<Contrast>::default())
            .add_plugin(ModifierPlugin::<Invert>::default())
            .add_plugin(ModifierPlugin::<Blur>::default())
            .add_plugin(ModifierPlugin::<Resize>::default())
            .add_plugin(SimplePencilPlugin)
            .add_plugin(RainbowPencilPlugin)
            .add_plugin(SortPencilPlugin);
    }
}

pub trait ModInstancer: Fn() -> Box<dyn Modifier + Send + Sync + 'static> + DynClone {
    fn instance(&self) -> Box<dyn Modifier + Send + Sync>;
}

impl<T: Fn() -> Box<dyn Modifier + Send + Sync + 'static> + DynClone> ModInstancer for T {
    fn instance(&self) -> Box<dyn Modifier + Send + Sync> {
        self()
    }
}

dyn_clone::clone_trait_object!(ModInstancer);

#[derive(Clone)]
pub struct ModifierIndex {
    pub name: String,
    pub id: TypeId,
    pub instancer: Box<dyn ModInstancer + Send + Sync + 'static>,
}

impl PartialEq for ModifierIndex {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Resource, Default)]
pub struct ModifierCollection {
    pub list: Vec<ModifierIndex>,
}
