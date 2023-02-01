use egui::{Color32, Context, Ui};
use glam::UVec2;

use crate::{
    color::Color,
    image::Image,
    modifier::{modification::Output, traits::Modifier},
    slot::ModifierSlot,
    view::View,
};

#[derive(Clone, PartialEq)]
pub struct MagicWand {
    pub target: Option<UVec2>,
    pub child: ModifierSlot,
    pub threshold: f32,
}

impl Default for MagicWand {
    fn default() -> Self {
        Self {
            target: Default::default(),
            child: Default::default(),
            threshold: 0.1,
        }
    }
}

impl MagicWand {
    pub fn update(&mut self, ctx: &Context, view: &View) {
        if ctx.input().pointer.primary_clicked() && !ctx.wants_pointer_input() {
            if let Some(pos) = view.hovered_pixel(ctx) {
                let egui::Vec2 { x, y } = pos;
                self.target = Some(UVec2::new(x.round() as u32, y.round() as u32));
            }
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

impl Modifier for MagicWand {
    fn apply(&mut self, mut input: Output) -> Option<Image> {
        if let Some(target) = self.target {
            if let Some(child) = self.child.get_mod_mut() {
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
