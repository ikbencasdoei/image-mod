use egui::{Context, Ui};
use glam::{UVec2, Vec2};

use crate::{
    color::Color,
    editor::Editor,
    image::Image,
    modifier::{modification::Output, traits::Modifier},
    view::View,
};

pub mod rainbow;
pub mod simple;
pub mod sort;

pub trait Pencil {
    fn pixel(&mut self, pixel: UVec2, image: &mut Image) -> Option<Color>;
    fn view(&mut self, _ui: &mut Ui) {}
}

#[derive(Clone, PartialEq, Default)]
pub struct PencilMod<T> {
    pixels: Vec<UVec2>,
    pencil: T,
    last_pixel: Option<Vec2>,
}

impl<T: Pencil + Default + PartialEq + Clone + 'static> Modifier for PencilMod<T> {
    fn apply(&mut self, mut input: Output) -> Option<Image> {
        if let Some(image) = &mut input.image {
            let mut pencil = self.pencil.clone();
            for pixel in self.pixels.iter() {
                if let Some(color) = pencil.pixel(*pixel, image) {
                    image.set_pixel(*pixel, color).ok();
                }
            }
        }
        input.image
    }

    fn view(&mut self, ui: &mut Ui, _: &mut Editor) {
        self.pencil.view(ui);
    }
}

impl<T: Pencil + Default + PartialEq + Clone> PencilMod<T> {
    pub fn update(&mut self, ctx: &Context, view: &View) {
        if (ctx.input().pointer.primary_down()) && !ctx.wants_pointer_input() {
            if let Some(pos) = view.hovered_pixel(ctx) {
                let egui::Vec2 { x, y } = pos;
                let pixel = Vec2::new(x, y);

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
            }
        } else {
            self.last_pixel = None;
        }
    }
}
