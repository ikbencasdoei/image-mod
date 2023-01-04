use std::f32::consts::PI;

use crate::prelude::{Color, Image};
use bevy::prelude::{Color as BevyColor, *};
use bevy_egui::egui::{self, Ui};

use super::plugin::{Pencil, PencilPlugin};

pub struct SortPencilPlugin;

impl Plugin for SortPencilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PencilPlugin::<PixelSorter>::default());
    }
}

#[derive(Clone, PartialEq)]
enum SortDirection {
    Forward,
    Backward,
    Both,
}

#[derive(Clone, PartialEq)]
pub struct PixelSorter {
    threshold: f32,
    angle: f32,
    direction: SortDirection,
}

impl Default for PixelSorter {
    fn default() -> Self {
        Self {
            angle: PI / 2.0,
            threshold: 0.1,
            direction: SortDirection::Both,
        }
    }
}

impl Pencil for PixelSorter {
    fn get_pixel(&mut self, pixel: UVec2, image: &mut Image) -> Option<Color> {
        let mut positions = Vec::<Vec2>::new();

        let direction = Vec2::new(self.angle.cos(), self.angle.sin());

        let mut current_position = pixel.as_vec2();

        if image.get_pixel(current_position.as_uvec2()).is_ok() {
            positions.push(current_position);
        }

        if self.direction == SortDirection::Forward || self.direction == SortDirection::Both {
            loop {
                let next_position = current_position - direction;

                let Ok(current_color) = image.get_pixel(current_position.as_uvec2()) else {
                    break;
                };

                let Ok(next_color) = image.get_pixel(next_position.as_uvec2()) else {
                    break;
                };

                if (current_color.sum() - next_color.sum()).abs() < self.threshold {
                    positions.push(next_position);
                } else {
                    break;
                }

                current_position = next_position;
            }
        }

        if self.direction == SortDirection::Backward || self.direction == SortDirection::Both {
            current_position = pixel.as_vec2();
            loop {
                let next_position = current_position + direction;

                let Ok(current_color) = image.get_pixel(current_position.as_uvec2()) else {
                    break;
                };

                let Ok(next_color) = image.get_pixel(next_position.as_uvec2()) else {
                    break;
                };

                if (current_color.sum() - next_color.sum()).abs() < self.threshold {
                    positions.push(next_position);
                } else {
                    break;
                }

                current_position = next_position;
            }
        }

        positions.sort_by(|a, b| {
            (direction.dot(*a) * a.length())
                .total_cmp(&(direction.dot(*b) * b.length()))
                .reverse()
        });

        let mut colors = Vec::<Color>::new();

        for position in &positions {
            colors.push(image.get_pixel(position.as_uvec2()).unwrap());
        }

        colors.sort_by(|a, b| a.sum().total_cmp(&b.sum()));

        for (position, color) in positions.iter().zip(colors.iter()) {
            image.set_pixel(position.as_uvec2(), *color).unwrap();
        }

        None
    }

    fn view(&mut self, ui: &mut Ui) {
        egui::Grid::new("")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                {
                    ui.label("threshold:");
                    ui.add(
                        egui::DragValue::new(&mut self.threshold)
                            .speed(0.01)
                            .clamp_range(0.0..=Color::from(BevyColor::WHITE).sum()),
                    );
                    ui.end_row();
                }

                {
                    let mut degrees = self.angle.to_degrees();

                    ui.label("angle:");
                    ui.add(
                        egui::DragValue::new(&mut degrees)
                            .speed(1)
                            .clamp_range(-360.0..=360.0)
                            .suffix("°"),
                    );

                    self.angle = degrees.to_radians();
                    ui.end_row();
                }

                {
                    ui.label("direction:");
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.direction, SortDirection::Forward, "Forward");
                        ui.selectable_value(&mut self.direction, SortDirection::Backward, "Back");
                        ui.selectable_value(&mut self.direction, SortDirection::Both, "Both");
                    });
                    ui.end_row();
                }
            });
    }
}
