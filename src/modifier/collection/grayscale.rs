use crate::{image::Image, modifier::modifier::Modifier};

#[derive(Clone, Default, PartialEq)]
pub struct GrayScaleFilter;

impl Modifier for GrayScaleFilter {
    fn apply(&mut self, mut input: Option<Image>) -> Option<Image> {
        if let Some(image) = &mut input {
            image.grayscale();
        }
        input
    }
}
