use egui::{Color32, Context, Ui};
use glam::UVec2;

use crate::{
    color::Color,
    editor::Editor,
   
    modifier::{cation::Output, traits::Modifier},
    slot::ModifierSlot,
    view::View,
};

#[derive(Clone, PartialEq)]
pub struct MagicWand {
    pub target: Option<UVec2>,
    pub input: ModifierSlot,
    pub threshold: f32,
}

impl Default for MagicWand {
    fn default() -> Self {
        Self {
            target: Default::default(),
            input: Default::default(),
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
            ui.label("threshold:");
            ui.add(
                egui::DragValue::new(&mut self.threshold)
                    .speed(0.001)
                    .clamp_range(0.0..=Color::from(Color32::WHITE).sum_rgb()),
            );
        });
    }
}

impl Modifier for MagicWand {
    fn apply(&mut self, input: &mut Output) {
        if let Some(target) = self.target {
            if let Some(child) = self.input.mod_mut() {
                if let Some(output) = child.output(&input).image.clone() {
                    if let Some(input) = &mut input.image {
                        let mut pixels = Vec::new();

                        if let Ok(target) = input.pixel_at(target) {
                            for pixel in input.iter_coords() {
                                let color = input.pixel_at(pixel).unwrap();
                                if (target.sum_rgb() - color.sum_rgb()).abs() < self.threshold {
                                    pixels.push(pixel);
                                }
                            }
                        }

                        for pixel in pixels {
                            let color = output.pixel_at(pixel).unwrap();
                            input.set_pixel(pixel, color).ok();
                        }
                    }
                }
            }
        }
    }

    fn view(&mut self, ui: &mut Ui, editor: &mut Editor) {
        self.view_threshold(ui);
        ui.label("input:");
        self.input.view_with_frame(ui, editor, None);

        if editor.is_modifier_selected::<Self>() {
            self.update(ui.ctx(), &editor.view);
        }
    }
}
