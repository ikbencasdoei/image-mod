use std::f32::consts::PI;

use bevy::prelude::{Color as BevyColor, *};
use bevy_egui::{
    egui::{self},
    EguiContext,
};

use crate::{
    color::Color,
    image::ImageHelper,
    tools::plugin::{Tool, ToolDescription},
};

use super::plugin::{PencilPlugin, PencilTool};

pub struct SortPencilPlugin;

impl Plugin for SortPencilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PencilPlugin::<PixelSorter>::default())
            .add_system(ui);
    }
}

#[derive(PartialEq)]
enum SortDirection {
    Forward,
    Backward,
    Both,
}

#[derive(Component, Reflect)]
pub struct PixelSorter {
    threshold: f32,
    angle: f32,
    #[reflect(ignore)]
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

impl Tool<PixelSorter> for PixelSorter {
    fn get_description() -> ToolDescription {
        ToolDescription {
            name: "Pixel Sort".to_string(),
        }
    }
}

impl PencilTool for PixelSorter {
    fn get_draw_color(&mut self, position: Vec2, image: &mut ImageHelper) -> Option<Color> {
        let mut positions = Vec::<Vec2>::new();

        let direction = Vec2::new(self.angle.cos(), self.angle.sin());

        let mut current_position = position;

        if image.get_pixel(current_position).is_ok() {
            positions.push(current_position);
        }

        if self.direction == SortDirection::Forward || self.direction == SortDirection::Both {
            loop {
                let next_position = current_position - direction;

                let Ok(current_color) = image.get_pixel(current_position) else {
                    break;
                };

                let Ok(next_color) = image.get_pixel(next_position) else {
                    break;
                };

                if (current_color.as_rgba_linear().sum() - next_color.as_rgba_linear().sum()).abs()
                    < self.threshold
                {
                    positions.push(next_position);
                } else {
                    break;
                }

                current_position = next_position;
            }
        }

        if self.direction == SortDirection::Backward || self.direction == SortDirection::Both {
            current_position = position;
            loop {
                let next_position = current_position + direction;

                let Ok(current_color) = image.get_pixel(current_position) else {
                    break;
                };

                let Ok(next_color) = image.get_pixel(next_position) else {
                    break;
                };

                if (current_color.as_rgba_linear().sum() - next_color.as_rgba_linear().sum()).abs()
                    < self.threshold
                {
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
            colors.push(image.get_pixel(*position).unwrap());
        }

        colors.sort_by(|a, b| a.sum().total_cmp(&b.sum()));

        for (position, color) in positions.iter().zip(colors.iter()) {
            image.set_pixel(*position, *color).ok();
        }

        None
    }
}

fn ui(mut egui_context: ResMut<EguiContext>, mut query: Query<&mut PixelSorter>) {
    for mut pencil in query.iter_mut() {
        egui::Window::new(PixelSorter::get_description().name).show(egui_context.ctx_mut(), |ui| {
            egui::Grid::new("")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    {
                        ui.label("threshold:");
                        ui.add(
                            egui::DragValue::new(&mut pencil.threshold)
                                .speed(0.01)
                                .clamp_range(0.0..=Color::from(BevyColor::WHITE).sum()),
                        );
                        ui.end_row();
                    }

                    {
                        let mut degrees = pencil.angle.to_degrees();

                        ui.label("angle:");
                        ui.add(
                            egui::DragValue::new(&mut degrees)
                                .speed(1)
                                .clamp_range(-360.0..=360.0)
                                .suffix("°"),
                        );

                        pencil.angle = degrees.to_radians();
                        ui.end_row();
                    }

                    {
                        ui.label("direction:");
                        ui.horizontal(|ui| {
                            ui.selectable_value(
                                &mut pencil.direction,
                                SortDirection::Forward,
                                "Forward",
                            );
                            ui.selectable_value(
                                &mut pencil.direction,
                                SortDirection::Backward,
                                "Back",
                            );
                            ui.selectable_value(&mut pencil.direction, SortDirection::Both, "Both");
                        });
                        ui.end_row();
                    }
                });
        });
    }
}
