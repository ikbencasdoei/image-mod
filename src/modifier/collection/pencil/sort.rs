use std::f32::consts::PI;

use egui::{Color32, Ui};
use glam::{UVec2, Vec2};

use super::Pencil;
use crate::{color::Color, image::Image};

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
    fn pixel(&mut self, pixel: UVec2, image: &mut Image) -> Option<Color> {
        let mut positions = Vec::<Vec2>::new();

        let direction = Vec2::new(self.angle.cos(), self.angle.sin());

        let mut current_position = pixel.as_vec2();

        if image.pixel(current_position.as_uvec2()).is_ok() {
            positions.push(current_position);
        }

        if self.direction == SortDirection::Forward || self.direction == SortDirection::Both {
            loop {
                let next_position = current_position - direction;

                let Ok(current_color) = image.pixel_vec(current_position) else {
                    break;
                };

                let Ok(next_color) = image.pixel_vec(next_position) else {
                    break;
                };

                if (current_color.sum_rgb() - next_color.sum_rgb()).abs() < self.threshold {
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

                let Ok(current_color) = image.pixel_vec(current_position) else {
                    break;
                };

                let Ok(next_color) = image.pixel_vec(next_position) else {
                    break;
                };

                if (current_color.sum_rgb() - next_color.sum_rgb()).abs() < self.threshold {
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
            colors.push(image.pixel_vec(*position).unwrap());
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
                    ui.add(
                        egui::DragValue::new(&mut self.threshold)
                            .speed(0.01)
                            .clamp_range(0.0..=Color::from(Color32::WHITE).sum_rgb()),
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
