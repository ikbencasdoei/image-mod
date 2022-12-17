use std::any::TypeId;

use bevy::prelude::*;
use dyn_clone::DynClone;

use crate::prelude::*;

pub struct SelectorCollectionPlugin;

impl Plugin for SelectorCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectorCollection>()
            .add_plugin(SelectorPlugin::<CanvasSelection>::default());
    }
}

pub trait SelInstancer: Fn() -> Box<dyn Selector + Send + Sync + 'static> + DynClone {
    fn instance(&self) -> Box<dyn Selector + Send + Sync>;
}

impl<T: Fn() -> Box<dyn Selector + Send + Sync + 'static> + DynClone> SelInstancer for T {
    fn instance(&self) -> Box<dyn Selector + Send + Sync> {
        self()
    }
}

dyn_clone::clone_trait_object!(SelInstancer);

#[derive(Clone)]
pub struct SelectorIndex {
    pub name: String,
    pub id: TypeId,
    pub instancer: Box<dyn SelInstancer + Send + Sync + 'static>,
}

impl PartialEq for SelectorIndex {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Resource, Default)]
pub struct SelectorCollection {
    pub list: Vec<SelectorIndex>,
}
