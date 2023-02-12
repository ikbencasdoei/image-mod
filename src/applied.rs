use std::ops::{Deref, DerefMut};

use eframe::emath::Numeric;
use egui::Response;

#[derive(Clone, Default)]
pub struct AppliedValue<T> {
    applied: T,
    temp: T,
}

impl<T: Clone> AppliedValue<T> {
    pub fn commit(&mut self) {
        self.applied = self.temp.clone()
    }
}

impl<T> Deref for AppliedValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.applied
    }
}

impl<T> DerefMut for AppliedValue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.temp
    }
}

impl<T: PartialEq> PartialEq for AppliedValue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.applied == other.applied
    }
}

impl<T: Numeric> AppliedValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            applied: value,
            temp: value,
        }
    }

    pub fn view(&mut self, closure: impl FnOnce(&mut T) -> Response) {
        let response = closure(&mut self.temp);

        if response.drag_released() || response.lost_focus() {
            self.commit()
        }
    }
}
