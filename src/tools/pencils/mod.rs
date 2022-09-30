use bevy::prelude::*;

pub mod plugin;
pub mod rainbow;
pub mod simple;
pub mod sort;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(simple::Plugin)
            .add_plugin(rainbow::Plugin)
            .add_plugin(sort::Plugin);
    }
}
