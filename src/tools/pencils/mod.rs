use bevy::prelude::*;

mod plugin;
mod rainbow;
mod simple;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(simple::Plugin);
    }
}
