use egui::{
    Color32, ColorImage, Context, Frame, Pos2, Rect, TextureFilter, TextureHandle, TextureOptions,
    Ui, Vec2,
};

use crate::image::Image;

pub struct View {
    texture: Option<TextureHandle>,
    pub scale: f32,
    pub translation: Vec2,
    dragging: bool,
    pub rect: Rect,
}

impl Default for View {
    fn default() -> Self {
        Self {
            texture: Default::default(),
            scale: 1.0,
            translation: Vec2::ZERO,
            dragging: false,
            rect: Rect::NAN,
        }
    }
}

impl View {
    pub fn update(&mut self, ctx: &Context, image: &Image) {
        const OPTIONS: TextureOptions = TextureOptions {
            magnification: TextureFilter::Nearest,
            minification: TextureFilter::Linear,
        };

        let image = image.as_rgba8();
        let size = [image.width() as usize, image.height() as usize];
        let pixels = image.as_flat_samples();

        let image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

        if let Some(texture) = &mut self.texture {
            texture.set(image, OPTIONS);
        } else {
            self.texture = Some(ctx.load_texture("image", image, OPTIONS));
        }
    }

    pub fn process(&mut self, ctx: &Context) {
        egui::CentralPanel::default()
            .frame(Frame::central_panel(&ctx.style()).inner_margin(0.0))
            .show(ctx, |ui| {
                if self.texture.is_some() {
                    self.input_zoom(ui);
                    self.input_drag(ui);
                    self.view(ui);
                }
            });
    }

    fn input_zoom(&mut self, ui: &mut Ui) {
        const FACTOR: f32 = 1.3;

        if ui.rect_contains_pointer(ui.max_rect()) {
            let pointer = ui.input().pointer.interact_pos().unwrap();
            let center_pointer = pointer.to_vec2() - self.rect.center().to_vec2();

            let scrolled = ui.ctx().input().scroll_delta.y;
            if scrolled.is_normal() {
                if scrolled.is_sign_positive() {
                    let diff = center_pointer * FACTOR - center_pointer;
                    self.translation -= diff;
                    self.scale *= FACTOR;
                } else {
                    let diff = center_pointer / FACTOR - center_pointer;
                    self.translation -= diff;
                    self.scale /= FACTOR;
                }
            }
        }
    }

    fn input_drag(&mut self, ui: &mut Ui) {
        if ui.rect_contains_pointer(ui.max_rect()) {
            if ui.ctx().input().pointer.any_pressed() && ui.ctx().input().pointer.middle_down() {
                self.dragging = true;
            } else if ui.ctx().input().pointer.any_released() {
                self.dragging = false;
            }
        }

        if self.dragging && ui.ctx().input().pointer.any_down() {
            self.translation += ui.ctx().input().pointer.delta();
        } else {
            self.dragging = false;
        }
    }

    fn view(&mut self, ui: &mut Ui) {
        let texture = self.texture.as_ref().unwrap();
        let size = texture.size_vec2() / ui.ctx().pixels_per_point() * self.scale;
        let center = ui.max_rect().center() + self.translation;
        self.rect = Rect::from_center_size(center, size);

        ui.painter().with_clip_rect(ui.max_rect()).image(
            texture.id(),
            self.rect,
            Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );
    }

    pub fn hovered_pixel(&self, ctx: &Context) -> Option<Vec2> {
        let pointer = ctx.input().pointer.interact_pos()?;
        if self.rect.contains(pointer) {
            let pos = pointer - self.rect.left_top();
            Some(pos / self.scale * ctx.pixels_per_point())
        } else {
            None
        }
    }
}
