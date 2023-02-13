use self::{
    blur::Blur,
    brighten::Brighten,
    bucket::Bucket,
    contrast::Contrast,
    fill::Fill,
    grayscale::GrayScaleFilter,
    hue::Hue,
    invert::Invert,
    list::List,
    magic_wand::MagicWand,
    overlay::Overlay,
    pencil::{rainbow::RainbowPencil, simple::SimplePencil, sort::PixelSorter, PencilMod},
    resize::Resize,
    source::Source,
};
use super::traits::Modifier;
use crate::editor::Editor;

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
pub mod overlay;
pub mod pencil;
pub mod resize;
pub mod source;

pub fn init_modifiers_collection(editor: &mut Editor) {
    editor.add_index(GrayScaleFilter::index());
    editor.add_index(Source::index());
    editor.add_index(Hue::index());
    editor.add_index(Brighten::index());
    editor.add_index(Contrast::index());
    editor.add_index(Invert::index());
    editor.add_index(Blur::index());
    editor.add_index(Resize::index());
    editor.add_index(Bucket::index());
    editor.add_index(Fill::index());
    editor.add_index(MagicWand::index());
    editor.add_index(Overlay::index());
    editor.add_index(List::index());
    editor.add_index(PencilMod::<SimplePencil>::index());
    editor.add_index(PencilMod::<RainbowPencil>::index());
    editor.add_index(PencilMod::<PixelSorter>::index());
}
