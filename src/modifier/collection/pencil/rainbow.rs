use super::Pencil;
use crate::{color::Color, image::Image, position::Position};

#[derive(Clone, PartialEq)]
pub struct RainbowPencil {
    color_hsv: (f32, f32, f32),
    rotation_per_pixel: f32,
    last_pixel: Option<Position>,
}

impl Default for RainbowPencil {
    fn default() -> Self {
        Self {
            color_hsv: (0.0, 1.0, 1.0),
            rotation_per_pixel: 0.01,
            last_pixel: None,
        }
    }
}

impl Pencil for RainbowPencil {
    fn pixel(&mut self, pixel: Position, _: &mut Image) -> Option<Color> {
        if let Some(last_pixel) = self.last_pixel {
            if pixel != last_pixel {
                self.color_hsv.0 += self.rotation_per_pixel;
            }
        }

        self.last_pixel = Some(pixel);

        Some(Color::from_hsv(
            self.color_hsv.0,
            self.color_hsv.2,
            self.color_hsv.2,
        ))
    }

    fn view(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                {
                    let mut degrees = self.color_hsv.0 % 1.0 * 360.0;

                    ui.label("current hue rotation:");
                    ui.add(
                        egui::DragValue::new(&mut degrees)
                            .speed(1.0)
                            .clamp_range(360.0..=0.0)
                            .suffix("°"),
                    );
                    self.color_hsv.0 = degrees / 360.0;
                }
                ui.end_row();

                {
                    let mut rotation_degrees = self.rotation_per_pixel * 360.0;

                    ui.label("hue rotation per pixel:");
                    ui.add(
                        egui::DragValue::new(&mut rotation_degrees)
                            .speed(0.01)
                            .clamp_range(360.0..=0.0)
                            .suffix("°"),
                    );

                    self.rotation_per_pixel = rotation_degrees / 360.0;
                }
                ui.end_row();
            });
    }
}
