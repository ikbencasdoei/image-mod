use std::path::Path;

use glam::{IVec2, UVec2, Vec2};
use image::{
    imageops::{self, FilterType},
    DynamicImage, ImageError, Rgba, RgbaImage,
};

use crate::color::Color;

#[derive(Clone)]
pub struct Image {
    image: RgbaImage,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            image: RgbaImage::new(1, 1),
        }
    }
}

impl Image {
    pub fn from_dyn(image: DynamicImage) -> Self {
        Self {
            image: image.into_rgba8(),
        }
    }

    pub fn into_dyn(self) -> DynamicImage {
        DynamicImage::ImageRgba8(self.image)
    }

    pub fn as_rgba8(&self) -> &RgbaImage {
        &self.image
    }

    pub fn set_pixel_vec(&mut self, position: Vec2, color: Color) -> Result<(), &str> {
        self.set_pixel_ivec(
            IVec2::new(position.x.round() as i32, position.y.round() as i32),
            color,
        )
    }

    pub fn set_pixel_ivec(&mut self, position: IVec2, color: Color) -> Result<(), &str> {
        if position.x.is_negative() || position.y.is_negative() {
            Err("negative value")
        } else {
            self.set_pixel(position.as_uvec2(), color)
        }
    }

    pub fn set_pixel(&mut self, position: UVec2, color: Color) -> Result<(), &str> {
        if self.contains_pixel(position) {
            self.image
                .put_pixel(position.x, position.y, Rgba(color.into_rgba_u8()));
            Ok(())
        } else {
            Err("pixel outside image")
        }
    }

    pub fn contains_pixel(&self, position: UVec2) -> bool {
        let size = self.size();
        (0..size.x).contains(&position.x) && (0..size.y).contains(&position.y)
    }

    pub fn size(&self) -> UVec2 {
        let (x, y) = self.image.dimensions();
        UVec2::new(x, y)
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = UVec2> {
        let size = self.size();

        (0..(size.x * size.y)).map(move |a| UVec2::new(a % size.x, a / size.x))
    }

    pub fn pixel_at_vec2(&self, position: Vec2) -> Result<Color, &str> {
        self.pixel_at_ivec(IVec2::new(
            position.x.round() as i32,
            position.y.round() as i32,
        ))
    }

    pub fn pixel_at_ivec(&self, position: IVec2) -> Result<Color, &str> {
        if position.x.is_negative() || position.y.is_negative() {
            Err("negative value")
        } else {
            self.pixel_at(position.as_uvec2())
        }
    }

    pub fn pixel_at(&self, position: UVec2) -> Result<Color, &str> {
        if self.contains_pixel(position) {
            let Rgba([r, g, b, a]) = *self.image.get_pixel(position.x, position.y);
            Ok(Color::from_rgba_u8(r, g, b, a))
        } else {
            Err("pixel outside image")
        }
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        Ok(Self::from_dyn(image::open(path)?))
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), ImageError> {
        self.image.save(path)
    }

    pub fn grayscale(&mut self) {
        self.image = imageops::colorops::grayscale_with_type_alpha(&self.image);
    }

    pub fn huerotate(&mut self, degrees: i32) {
        imageops::colorops::huerotate_in_place(&mut self.image, degrees);
    }

    pub fn brighten(&mut self, value: i32) {
        imageops::colorops::brighten_in_place(&mut self.image, value)
    }

    pub fn contrast(&mut self, value: f32) {
        imageops::colorops::contrast_in_place(&mut self.image, value)
    }

    pub fn invert(&mut self) {
        imageops::colorops::invert(&mut self.image);
    }

    pub fn blur(&mut self, sigma: f32) {
        self.image = imageops::blur(&self.image, sigma)
    }

    pub fn resize(&mut self, new_size: UVec2, filter: FilterType) {
        self.image = imageops::resize(&self.image, new_size.x, new_size.y, filter);
    }

    pub fn overlay(&mut self, overlay: &Image, position: IVec2) {
        imageops::overlay(
            &mut self.image,
            &overlay.image,
            position.x.into(),
            position.y.into(),
        )
    }
}
