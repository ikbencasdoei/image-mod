use crate::{image::Image, modifier::traits::Modifier};

#[derive(Clone, Default, PartialEq)]
pub struct Invert;

impl Modifier for Invert {
    fn apply(&mut self, mut input: Option<Image>) -> Option<Image> {
        if let Some(image) = &mut input {
            image.invert();
        }
        input
    }
}
