use std::any::{type_name, Any, TypeId};

use dyn_clone::DynClone;
use eframe::egui::Ui;

use super::cation::Output;
use crate::editor::Editor;

pub trait Modifier: DynClone + DynPartialEq {
    fn apply(&mut self, input: &mut Output);

    fn name() -> String
    where
        Self: Sized,
    {
        type_name::<Self>()
            .split_inclusive("<")
            .map(|a| a.split("::").last().unwrap())
            .collect::<Vec<&str>>()
            .concat()
    }

    fn index() -> ModifierIndex
    where
        Self: Sized + Default + 'static,
    {
        ModifierIndex {
            name: Self::name(),
            id: TypeId::of::<Self>(),
            instancer: Box::new(|| Box::<Self>::default()),
        }
    }

    #[allow(unused_variables)]
    fn view(&mut self, ui: &mut Ui, editor: &mut Editor) {}
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

pub trait ModInstancer: Fn() -> Box<dyn Modifier> + DynClone {
    fn instance(&self) -> Box<dyn Modifier>;
}

impl<T: Fn() -> Box<dyn Modifier> + DynClone> ModInstancer for T {
    fn instance(&self) -> Box<dyn Modifier> {
        self()
    }
}

dyn_clone::clone_trait_object!(ModInstancer);

#[derive(Clone)]
pub struct ModifierIndex {
    pub name: String,
    pub id: TypeId,
    pub instancer: Box<dyn ModInstancer>,
}

impl PartialEq for ModifierIndex {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
