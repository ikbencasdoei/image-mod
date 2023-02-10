use crate::{
    
    modifier::{cation::Output, traits::Modifier},
};

#[derive(Clone, Default, PartialEq)]
pub struct Invert;

impl Modifier for Invert {
    fn apply(&mut self, input: &mut Output) {
        if let Some(image) = &mut input.image {
            image.invert();
        }
        
    }
}
