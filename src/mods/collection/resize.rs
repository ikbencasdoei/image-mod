use bevy::prelude::*;
use bevy_egui::egui::{self, Ui};
use image::imageops::FilterType;

use crate::prelude::{Image, *};

#[derive(Clone, PartialEq)]
pub struct Resize {
    new_size: UVec2,
    filter: FilterType,
}

impl Default for Resize {
    fn default() -> Self {
        Self {
            new_size: UVec2::new(100, 100),
            filter: FilterType::Gaussian,
        }
    }
}

impl Modifier for Resize {
    fn apply(&mut self, mut input: Option<Image>) -> Option<Image> {
        if let Some(image) = &mut input {
            image.resize(self.new_size, self.filter);
        }
        input
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("new height:");
            ui.add(egui::DragValue::new(&mut self.new_size.x).clamp_range(1..=u32::MAX));
        });

        ui.horizontal(|ui| {
            ui.label("new width:");
            ui.add(egui::DragValue::new(&mut self.new_size.y).clamp_range(1..=u32::MAX));
        });

        ui.horizontal(|ui| {
            ui.label("filter:");
            egui::ComboBox::from_id_source("filter")
                .selected_text(format!("{}", display_filter(self.filter)))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.filter,
                        FilterType::Nearest,
                        display_filter(FilterType::Nearest),
                    );
                    ui.selectable_value(
                        &mut self.filter,
                        FilterType::Triangle,
                        display_filter(FilterType::Triangle),
                    );
                    ui.selectable_value(
                        &mut self.filter,
                        FilterType::CatmullRom,
                        display_filter(FilterType::CatmullRom),
                    );
                    ui.selectable_value(
                        &mut self.filter,
                        FilterType::Gaussian,
                        display_filter(FilterType::Gaussian),
                    );
                    ui.selectable_value(
                        &mut self.filter,
                        FilterType::Lanczos3,
                        display_filter(FilterType::Lanczos3),
                    );
                });
        });
    }
}

fn display_filter(filter: FilterType) -> &'static str {
    match filter {
        FilterType::Nearest => "Nearest Meighbor",
        FilterType::Triangle => "Linear",
        FilterType::CatmullRom => "Cubic",
        FilterType::Gaussian => "Gaussian",
        FilterType::Lanczos3 => "Lanczos",
    }
}
