use uuid::Uuid;

use crate::modifier::{
    modification::{Cacher, DynMod},
    traits::ModifierIndex,
};

#[derive(Default)]
pub struct Editor {
    pub index: Vec<ModifierIndex>,
    pub selected: Option<Uuid>,
    pub dragging: Option<Cacher<DynMod>>,
    pub dropped: Option<Cacher<DynMod>>,
}

impl Editor {
    pub fn add_index(&mut self, index: ModifierIndex) {
        self.index.push(index);
        self.index.sort_by(|a, b| a.name.cmp(&b.name));
    }
}
