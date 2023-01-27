use egui::Context;

use self::{
    blur::Blur,
    brighten::Brighten,
    bucket::Bucket,
    contrast::Contrast,
    fill::Fill,
    grayscale::GrayScaleFilter,
    hue::Hue,
    invert::Invert,
    pencil::{rainbow::RainbowPencil, simple::SimplePencil, sort::PixelSorter, PencilMod},
    resize::Resize,
    source::Source,
};
use super::traits::init_modifier;
use crate::{editor::Editor, project::Project, view::View};

pub mod blur;
pub mod brighten;
pub mod bucket;
pub mod contrast;
pub mod fill;
pub mod grayscale;
pub mod hue;
pub mod invert;
pub mod list;
pub mod magic_wand;
pub mod pencil;
pub mod resize;
pub mod source;

pub fn init_modifiers_collection(editor: &mut Editor) {
    init_modifier::<GrayScaleFilter>(editor);
    init_modifier::<Source>(editor);
    init_modifier::<Hue>(editor);
    init_modifier::<Brighten>(editor);
    init_modifier::<Contrast>(editor);
    init_modifier::<Invert>(editor);
    init_modifier::<Blur>(editor);
    init_modifier::<Resize>(editor);
    init_modifier::<Bucket>(editor);
    init_modifier::<Fill>(editor);
    init_modifier::<PencilMod<SimplePencil>>(editor);
    init_modifier::<PencilMod<RainbowPencil>>(editor);
    init_modifier::<PencilMod<PixelSorter>>(editor);
}

pub fn process_modifiers(project: &mut Project, ctx: &Context, view: &View, editor: &Editor) {
    if let Some(modification) = project.root.modifier.get_selected_mod_mut(editor) {
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

        if let Some(modifier) = modification.modifier.get_modifier_mut::<Bucket>() {
            modifier.update(ctx, view);
        }
    }
}
