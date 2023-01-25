use std::any::TypeId;

use dyn_clone::DynClone;
use egui::Context;

use self::{
    blur::Blur,
    brighten::Brighten,
    contrast::Contrast,
    grayscale::GrayScaleFilter,
    hue::Hue,
    invert::Invert,
    pencil::{plugin::PencilMod, rainbow::RainbowPencil, simple::SimplePencil, sort::PixelSorter},
    resize::Resize,
    source::Source,
};
use super::{
    modifier::{init_modifier, Modifier},
    ui::ModifierUi,
};
use crate::{project::Project, view::View};

pub mod blur;
pub mod brighten;
pub mod contrast;
pub mod grayscale;
pub mod hue;
pub mod invert;
pub mod list;
pub mod pencil;
pub mod resize;
pub mod source;

pub fn init_modifiers_collection(mod_ui: &mut ModifierUi) {
    init_modifier::<GrayScaleFilter>(mod_ui);
    init_modifier::<Source>(mod_ui);
    init_modifier::<Hue>(mod_ui);
    init_modifier::<Brighten>(mod_ui);
    init_modifier::<Contrast>(mod_ui);
    init_modifier::<Invert>(mod_ui);
    init_modifier::<Blur>(mod_ui);
    init_modifier::<Resize>(mod_ui);
    init_modifier::<PencilMod<SimplePencil>>(mod_ui);
    init_modifier::<PencilMod<RainbowPencil>>(mod_ui);
    init_modifier::<PencilMod<PixelSorter>>(mod_ui);
}

pub fn process_modifiers(project: &mut Project, ctx: &Context, view: &View) {
    if let Some(modification) = project.get_selected_mod_mut() {
        if let Some(modifier) = modification
            .modifier
            .get_modifier_mut::<PencilMod<SimplePencil>>()
        {
            modifier.update(ctx, view);
        }

        if let Some(modifier) = modification
            .modifier
            .get_modifier_mut::<PencilMod<RainbowPencil>>()
        {
            modifier.update(ctx, view);
        }

        if let Some(modifier) = modification
            .modifier
            .get_modifier_mut::<PencilMod<PixelSorter>>()
        {
            modifier.update(ctx, view);
        }
    }
}

pub trait ModInstancer: Fn() -> Box<dyn Modifier> + DynClone {
    fn instance(&self) -> Box<dyn Modifier>;
}

impl<T: Fn() -> Box<dyn Modifier> + DynClone> ModInstancer for T {
    fn instance(&self) -> Box<dyn Modifier> {
        self()
    }
}

dyn_clone::clone_trait_object!(ModInstancer);

#[derive(Clone)]
pub struct ModifierIndex {
    pub name: String,
    pub id: TypeId,
    pub instancer: Box<dyn ModInstancer>,
}

impl PartialEq for ModifierIndex {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
