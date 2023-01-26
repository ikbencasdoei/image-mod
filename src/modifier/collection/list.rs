use crate::{
    image::Image,
    modifier::{
        modification::{DynMod, ModOutput, Modification},
        traits::Modifier,
    },
};

#[derive(Default, Clone, PartialEq)]
pub struct List {
    pub contents: Vec<Modification<DynMod>>,
}

impl Modifier for List {
    fn apply(&mut self, input: Option<Image>) -> Option<Image> {
        let mut output = ModOutput::new(input);
        {
            let mut borrow = &output;
            for modification in self.contents.iter_mut() {
                borrow = modification.get_output(&borrow);
            }
            output = borrow.clone();
        }

        output.image
    }
}
