use bevy::prelude::*;

use self::{rainbow::RainbowPencilPlugin, simple::SimplePencilPlugin, sort::SortPencilPlugin};

pub mod plugin;
pub mod rainbow;
pub mod simple;
pub mod sort;

pub struct PencilBoxPlugin;
impl Plugin for PencilBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SimplePencilPlugin)
            .add_plugin(RainbowPencilPlugin)
            .add_plugin(SortPencilPlugin);
    }
}
