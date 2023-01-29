use crate::{
    image::Image,
    modifier::{modification::CacheOutput, traits::Modifier},
};

#[derive(Clone, Default, PartialEq)]
pub struct Invert;

impl Modifier for Invert {
    fn apply(&mut self, mut input: CacheOutput) -> Option<Image> {
        if let Some(image) = &mut input.image {
            image.invert();
        }
        input.image
    }
}
