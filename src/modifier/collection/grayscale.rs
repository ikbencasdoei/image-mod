use crate::{
    image::Image,
    modifier::{modification::CacheOutput, traits::Modifier},
};

#[derive(Clone, Default, PartialEq)]
pub struct GrayScaleFilter;

impl Modifier for GrayScaleFilter {
    fn apply(&mut self, mut input: CacheOutput) -> Option<Image> {
        if let Some(image) = &mut input.image {
            image.grayscale();
        }
        input.image
    }
}
