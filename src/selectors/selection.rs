use bevy::prelude::*;

use crate::prelude::{Image, *};

pub trait Selection: Reflect {
    fn get_pixels(&self, image: &Image) -> Vec<UVec2>;
    fn get_index() -> SelectorIndex
    where
        Self: Sized + Default,
    {
        SelectorIndex::from_type_name(Self::default().type_name())
    }
}
