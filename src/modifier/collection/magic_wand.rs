use egui::{Color32, Context, Ui};
use glam::UVec2;

use crate::{
    color::Color,
    image::Image,
    modifier::{
        modification::{ModOutput, Modification},
        traits::Modifier,
    },
    view::View,
};

#[derive(Clone, PartialEq, Default)]
pub struct MagicWand<T> {
    pub target: Option<UVec2>,
    pub threshold: f32,
    pub child: Option<Modification<T>>,
}

impl<T> MagicWand<T> {
    pub fn update(&mut self, ctx: &Context, view: &View) {
        if ctx.input().pointer.primary_clicked() && !ctx.wants_pointer_input() {
            let pixel = {
                let egui::Vec2 { x, y } = view.hovered_pixel(ctx);
                UVec2::new(x.round() as u32, y.round() as u32)
            };

            self.target = Some(pixel);
        }
    }

    pub fn view_threshold(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("threshold");
            ui.add(
                egui::DragValue::new(&mut self.threshold)
                    .speed(0.001)
                    .clamp_range(0.0..=Color::from(Color32::WHITE).sum_rgb()),
            );
        });
    }
}

impl<T: Modifier + Clone + PartialEq + 'static> Modifier for MagicWand<T> {
    fn apply(&mut self, mut input: ModOutput) -> Option<Image> {
        if let Some(target) = self.target {
            if let Some(child) = &mut self.child {
                if let Some(output) = child.get_output(&input).image.clone() {
                    if let Some(input) = &mut input.image {
                        let mut pixels = Vec::new();

                        if let Ok(target) = input.get_pixel(target) {
                            for pixel in input.iter_coords() {
                                let color = input.get_pixel(pixel).unwrap();
                                if (target.sum_rgb() - color.sum_rgb()).abs() < self.threshold {
                                    pixels.push(pixel);
                                }
                            }
                        }

                        for pixel in pixels {
                            let color = output.get_pixel(pixel).unwrap();
                            input.set_pixel(pixel, color).ok();
                        }
                    }
                }
            }
        }

        input.image
    }
}
