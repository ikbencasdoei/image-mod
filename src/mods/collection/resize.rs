use bevy::prelude::*;
use bevy_egui::egui::{self, Ui};
use image::imageops::FilterType;

use crate::prelude::{Image, *};

#[derive(Clone, PartialEq)]
pub struct Resize {
    size: Size,
    filter: FilterType,
}

#[derive(Clone, PartialEq)]
enum Size {
    Absolute(UVec2),
    Relative(Vec2),
}

impl Default for Resize {
    fn default() -> Self {
        Self {
            size: Size::Relative(Vec2::new(100.0, 100.0)),
            filter: FilterType::Gaussian,
        }
    }
}

impl Modifier for Resize {
    fn apply(&mut self, mut input: Option<Image>) -> Option<Image> {
        if let Some(image) = &mut input {
            match self.size {
                Size::Absolute(size) => image.resize(size, self.filter),
                Size::Relative(size) => {
                    let pixels = (image.size().as_vec2() * (size / 100.0))
                        .as_uvec2()
                        .max(UVec2::new(1, 1));
                    image.resize(pixels, self.filter)
                }
            }
        }
        input
    }

    fn view(&mut self, ui: &mut Ui) {
        match self.size {
            Size::Absolute(_) => {
                if ui.checkbox(&mut false, "relative").changed() {
                    self.size = Size::Relative(Vec2::new(100.0, 100.0))
                }
            }
            Size::Relative(_) => {
                if ui.checkbox(&mut true, "relative").changed() {
                    self.size = Size::Absolute(UVec2::new(100, 100))
                }
            }
        }

        match &mut self.size {
            Size::Absolute(size) => {
                ui.horizontal(|ui| {
                    ui.label("new height:");
                    ui.add(
                        egui::DragValue::new(&mut size.x)
                            .clamp_range(1..=u32::MAX)
                            .suffix("px"),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("new width:");
                    ui.add(
                        egui::DragValue::new(&mut size.y)
                            .clamp_range(1..=u32::MAX)
                            .suffix("px"),
                    );
                });
            }
            Size::Relative(size) => {
                ui.horizontal(|ui| {
                    ui.label("new height:");
                    ui.add(
                        egui::DragValue::new(&mut size.x)
                            .clamp_range(f32::MIN_POSITIVE..=f32::MAX)
                            .suffix("%"),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("new width:");
                    ui.add(
                        egui::DragValue::new(&mut size.y)
                            .clamp_range(f32::MIN_POSITIVE..=f32::MAX)
                            .suffix("%"),
                    );
                });
            }
        }

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
