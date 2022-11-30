use bevy::prelude::*;

use crate::prelude::*;

pub mod plugin;
pub mod rainbow;
pub mod simple;
pub mod sort;

pub struct PencilCollection;
impl Plugin for PencilCollection {
    fn build(&self, app: &mut App) {
        app.add_plugin(SimplePencilPlugin)
            .add_plugin(RainbowPencilPlugin)
            .add_plugin(SortPencilPlugin);
    }
}
