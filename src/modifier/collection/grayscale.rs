use crate::modifier::{cation::Output, traits::Modifier};

#[derive(Clone, Default, PartialEq)]
pub struct GrayScaleFilter;

impl Modifier for GrayScaleFilter {
    fn apply(&mut self, input: &mut Output) {
        if let Some(image) = &mut input.image {
            image.grayscale();
        }
    }
}
