use std::f32::consts::PI;

use egui::{Color32, Ui};
use glam::{UVec2, Vec2};

use super::Pencil;
use crate::{applied::AppliedValue, color::Color, image::Image};

#[derive(Clone, PartialEq)]
enum SortDirection {
    Forward,
    Backward,
    Both,
}

#[derive(Clone, PartialEq)]
pub struct PixelSorter {
    threshold: AppliedValue<f32>,
    angle: AppliedValue<f32>,
    direction: SortDirection,
}

impl Default for PixelSorter {
    fn default() -> Self {
        Self {
            angle: AppliedValue::new(PI / 2.0),
            threshold: AppliedValue::new(0.1),
            direction: SortDirection::Both,
        }
    }
}

impl Pencil for PixelSorter {
    fn pixel(&mut self, pixel: UVec2, image: &mut Image) -> Option<Color> {
        let mut positions = Vec::<Vec2>::new();

        let direction = Vec2::new(self.angle.cos(), self.angle.sin());

        let mut current_position = pixel.as_vec2();

        if image.pixel_at(current_position.as_uvec2()).is_ok() {
            positions.push(current_position);
        }

        if self.direction == SortDirection::Forward || self.direction == SortDirection::Both {
            loop {
                let next_position = current_position - direction;

                let Ok(current_color) = image.pixel_at_vec2(current_position) else {
                    break;
                };

                let Ok(next_color) = image.pixel_at_vec2(next_position) else {
                    break;
                };

                if (current_color.sum_rgb() - next_color.sum_rgb()).abs() < *self.threshold {
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

                let Ok(current_color) = image.pixel_at_vec2(current_position) else {
                    break;
                };

                let Ok(next_color) = image.pixel_at_vec2(next_position) else {
                    break;
                };

                if (current_color.sum_rgb() - next_color.sum_rgb()).abs() < *self.threshold {
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
            colors.push(image.pixel_at_vec2(*position).unwrap());
        }

        colors.sort_by(|a, b| a.sum_rgb().total_cmp(&b.sum_rgb()));

        for (position, color) in positions.iter().zip(colors.iter()) {
            image.set_pixel_vec(*position, *color).unwrap();
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
                    self.threshold.view(|value| {
                        ui.add(
                            egui::DragValue::new(value)
                                .speed(0.01)
                                .clamp_range(0.0..=Color::from(Color32::WHITE).sum_rgb()),
                        )
                    });

                    ui.end_row();
                }

                self.angle.view(|value| {
                    let mut degrees = value.to_degrees();

                    ui.label("angle:");
                    let response = ui.add(
                        egui::DragValue::new(&mut degrees)
                            .speed(1)
                            .clamp_range(-360.0..=360.0)
                            .suffix("°"),
                    );

                    *value = degrees.to_radians();
                    ui.end_row();
                    response
                });

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
