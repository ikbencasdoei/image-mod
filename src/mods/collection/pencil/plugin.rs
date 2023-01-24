use egui::{Context, Ui};
use glam::{UVec2, Vec2};

use crate::{color::Color, image::Image, mods::plugin::Modifier, view::View};

pub trait Pencil {
    fn get_pixel(&mut self, pixel: UVec2, image: &mut Image) -> Option<Color>;
    fn view(&mut self, _ui: &mut Ui) {}
}

#[derive(Clone, PartialEq, Default)]
pub struct PencilMod<T> {
    pixels: Vec<UVec2>,
    pencil: T,
    last_pixel: Option<Vec2>,
}

impl<T: Pencil + Default + PartialEq + Clone + 'static> Modifier for PencilMod<T> {
    fn apply(&mut self, mut input: Option<Image>) -> Option<Image> {
        if let Some(image) = &mut input {
            let mut pencil = self.pencil.clone();
            for pixel in self.pixels.iter() {
                if let Some(color) = pencil.get_pixel(*pixel, image) {
                    image.set_pixel(*pixel, color).ok();
                }
            }
        }
        input
    }

    fn view(&mut self, ui: &mut Ui) {
        self.pencil.view(ui);
    }
}

impl<T: Pencil + Default + PartialEq + Clone + 'static> PencilMod<T> {
    pub fn update(&mut self, ctx: &Context, view: &View) {
        if (ctx.input().pointer.primary_down()) && !ctx.wants_pointer_input() {
            let pixel = {
                let egui::Vec2 { x, y } = view.hovered_pixel(ctx);
                Vec2::new(x, y)
            };

            if let Some(last_pixel) = self.last_pixel {
                let delta: Vec2 = pixel - last_pixel;

                if delta.length() > 1.0 {
                    for i in 1..delta.length().ceil() as i32 {
                        let position =
                            last_pixel.lerp(pixel, 1.0 / delta.length().ceil() * (i as f32));

                        self.pixels.push(position.as_uvec2());
                    }
                }
            }

            self.pixels.push(pixel.as_uvec2());

            self.last_pixel = Some(pixel);
        } else {
            self.last_pixel = None;
        }
    }
}
