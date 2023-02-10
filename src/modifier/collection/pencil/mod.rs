use egui::Ui;
use glam::{UVec2, Vec2};
use uuid::Uuid;

use crate::{
    color::Color,
    editor::Editor,
    image::Image,
    modifier::{cation::Output, traits::Modifier},
};

pub mod rainbow;
pub mod simple;
pub mod sort;

pub trait Pencil {
    fn pixel(&mut self, pixel: UVec2, image: &mut Image) -> Option<Color>;
    fn view(&mut self, _ui: &mut Ui) {}
}

#[derive(Clone, Default)]
pub struct PencilMod<T> {
    pixels: Vec<UVec2>,
    pencil: T,
    last_pixel: Option<Vec2>,
    cached: Option<Cached<T>>,
}

#[derive(Clone)]
struct Cached<T> {
    pixels: Vec<UVec2>,
    image: Option<Image>,
    pencil: T,
    input_id: Uuid,
}

impl<T: PartialEq> PartialEq for PencilMod<T> {
    fn eq(&self, other: &Self) -> bool {
        self.pixels == other.pixels && self.pencil == other.pencil
    }
}

impl<T: Pencil + Default + PartialEq + Clone + 'static> Modifier for PencilMod<T> {
    fn apply(&mut self, input: &mut Output) {
        let (mut prepared_pencil, mut prepared_image, prepared_pixels) =
            if self.cached.as_ref().is_some_and(|cache| {
                cache.input_id == input.id
                    && cache.pixels == self.pixels[0..cache.pixels.len()]
                    && cache.pencil == self.pencil
            }) {
                (
                    self.cached.as_ref().unwrap().pencil.clone(),
                    self.cached.as_ref().unwrap().image.clone(),
                    &self.pixels[self.cached.as_ref().unwrap().pixels.len()..self.pixels.len()],
                )
            } else {
                (
                    self.pencil.clone(),
                    input.image.clone(),
                    self.pixels.as_slice(),
                )
            };

        if let Some(image) = &mut prepared_image {
            for pixel in prepared_pixels.iter() {
                if let Some(color) = prepared_pencil.pixel(*pixel, image) {
                    image.set_pixel(*pixel, color).ok();
                }
            }
        }

        self.cached = Some(Cached {
            pixels: self.pixels.clone(),
            image: prepared_image.clone(),
            input_id: input.id,
            pencil: prepared_pencil,
        });

        input.image = prepared_image;
    }

    fn view(&mut self, ui: &mut Ui, editor: &mut Editor) {
        self.pencil.view(ui);

        if editor.is_modifier_selected::<Self>() {
            if (ui.ctx().input().pointer.primary_down()) && !ui.ctx().wants_pointer_input() {
                if let Some(pos) = editor.view.hovered_pixel(ui.ctx()) {
                    let egui::Vec2 { x, y } = pos;
                    let pixel = Vec2::new(x, y);

                    if let Some(last_pixel) = self.last_pixel {
                        let delta: Vec2 = pixel - last_pixel;

                        if delta.length() > 1.0 {
                            for i in 1..delta.length().ceil() as i32 {
                                let position = last_pixel
                                    .lerp(pixel, 1.0 / delta.length().ceil() * (i as f32));

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
}
