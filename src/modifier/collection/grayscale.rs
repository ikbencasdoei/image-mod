use crate::{
    image::Image,
    modifier::{cation::Output, traits::Modifier},
};

#[derive(Clone, Default, PartialEq)]
pub struct GrayScaleFilter;

impl Modifier for GrayScaleFilter {
    fn apply(&mut self, mut input: Output) -> Option<Image> {
        if let Some(image) = &mut input.image {
            image.grayscale();
        }
        input.image
    }
}
