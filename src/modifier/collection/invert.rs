use crate::{
    image::Image,
    modifier::{cation::Output, traits::Modifier},
};

#[derive(Clone, Default, PartialEq)]
pub struct Invert;

impl Modifier for Invert {
    fn apply(&mut self, mut input: Output) -> Option<Image> {
        if let Some(image) = &mut input.image {
            image.invert();
        }
        input.image
    }
}
