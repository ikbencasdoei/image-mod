use std::{
    any::{type_name, Any, TypeId},
    marker::PhantomData,
};

use bevy::prelude::*;
use bevy_egui::egui::Ui;
use dyn_clone::DynClone;

use super::{collection::ModifierIndex, ui::ModifierUi};
use crate::image::Image;

pub trait Modifier: DynClone + DynPartialEq {
    fn apply(&mut self, input: Option<Image>) -> Option<Image>;

    fn get_index() -> ModifierIndex
    where
        Self: Sized + Default + Send + Sync + 'static,
    {
        ModifierIndex {
            name: type_name::<Self>()
                .split("::")
                .last()
                .unwrap()
                .replace('>', ""),
            id: TypeId::of::<Self>(),
            instancer: Box::new(|| Box::<Self>::default()),
        }
    }

    fn view(&mut self, _ui: &mut Ui) {}
}

dyn_clone::clone_trait_object!(Modifier);

pub trait DynPartialEq {
    fn eq(&self, other: &dyn DynPartialEq) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl PartialEq for dyn DynPartialEq + '_ {
    fn eq(&self, other: &dyn DynPartialEq) -> bool {
        DynPartialEq::eq(self, other)
    }
}

impl<T: PartialEq + 'static> DynPartialEq for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq(&self, other: &dyn DynPartialEq) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.eq(other)
        } else {
            false
        }
    }
}

#[derive(Default)]
pub struct ModifierPlugin<T>(PhantomData<T>);

impl<T> Plugin for ModifierPlugin<T>
where
    T: Modifier + Default + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup::<T>);
    }
}

fn setup<T: Modifier + Default + Send + Sync + 'static>(mut mod_ui: ResMut<ModifierUi>) {
    mod_ui.add_index(T::get_index());
}
