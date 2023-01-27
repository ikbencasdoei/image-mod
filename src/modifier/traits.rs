use std::any::{type_name, Any, TypeId};

use dyn_clone::DynClone;
use egui::Ui;

use super::modification::ModOutput;
use crate::{editor::Editor, image::Image};

pub trait Modifier: DynClone + DynPartialEq {
    fn apply(&mut self, input: ModOutput) -> Option<Image>;

    fn get_name() -> String
    where
        Self: Sized,
    {
        type_name::<Self>()
            .split_inclusive("<")
            .map(|a| a.split("::").last().unwrap())
            .collect::<Vec<&str>>()
            .concat()
    }

    fn get_index() -> ModifierIndex
    where
        Self: Sized + Default + 'static,
    {
        ModifierIndex {
            name: Self::get_name(),
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

pub fn init_modifier<T: Modifier + Default + 'static>(editor: &mut Editor) {
    editor.add_index(T::get_index());
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
