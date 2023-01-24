use egui::{
    Color32, ColorImage, Context, Frame, Pos2, Rect, TextureFilter, TextureHandle, TextureOptions,
    Ui, Vec2,
};

use crate::image::Image;

pub struct View {
    texture: Option<TextureHandle>,
    scale: f32,
    translation: Vec2,
    dragging: bool,
}

impl Default for View {
    fn default() -> Self {
        Self {
            texture: Default::default(),
            scale: 1.0,
            translation: Vec2::ZERO,
            dragging: false,
        }
    }
}

impl View {
    pub fn update(&mut self, ctx: &Context, image: Image) {
        const OPTIONS: TextureOptions = TextureOptions {
            magnification: TextureFilter::Nearest,
            minification: TextureFilter::Linear,
        };

        let image = image.into_dyn().to_rgba8();

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
        if self.texture.is_some() {
            egui::CentralPanel::default()
                .frame(Frame::central_panel(&ctx.style()).inner_margin(0.0))
                .show(ctx, |ui| {
                    self.view(ctx, ui);
                    self.zoom(ctx, ui);
                    self.drag(ctx, ui);
                    // self.view(ctx, ui);
                });
        }
    }

    fn zoom(&mut self, ctx: &Context, ui: &mut Ui) {
        const FACTOR: f32 = 1.3;

        if ui.rect_contains_pointer(ui.max_rect()) {
            let pointer = ui.input().pointer.interact_pos().unwrap();
            let center_pointer = pointer.to_vec2() - self.rect(ctx, ui).center().to_vec2();

            let scrolled = ctx.input().scroll_delta.y;
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

    fn drag(&mut self, ctx: &Context, ui: &mut Ui) {
        if ui.rect_contains_pointer(ui.max_rect()) {
            {
                if ctx.input().pointer.any_pressed() && ctx.input().pointer.middle_down() {
                    self.dragging = true;
                } else if ctx.input().pointer.any_released() {
                    self.dragging = false;
                }
            }
        }

        if self.dragging && ctx.input().pointer.any_down() {
            self.translation += ctx.input().pointer.delta();
        } else {
            self.dragging = false;
        }
    }

    fn view(&self, ctx: &Context, ui: &mut Ui) {
        let texture = self.texture.as_ref().unwrap();
        ui.painter().image(
            texture.id(),
            self.rect(ctx, ui),
            Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        )
    }

    fn rect(&self, ctx: &Context, ui: &Ui) -> Rect {
        let texture = self.texture.as_ref().unwrap();
        let size = texture.size_vec2() / ctx.pixels_per_point() * self.scale;
        let center = ui.max_rect().center() + self.translation;
        Rect::from_center_size(center, size)
    }
}
