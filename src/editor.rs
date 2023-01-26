use uuid::Uuid;

use crate::modifier::collection::ModifierIndex;

#[derive(Default)]
pub struct Editor {
    pub index: Vec<ModifierIndex>,
    pub selected: Option<Uuid>,
    pub dragging: Option<Uuid>,
    pub removed: Option<Uuid>,
    pub dropped: Option<usize>,
}

impl Editor {
    pub fn add_index(&mut self, index: ModifierIndex) {
        self.index.push(index);
        self.index.sort_by(|a, b| a.name.cmp(&b.name));
    }
}
