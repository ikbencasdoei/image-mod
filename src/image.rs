use std::path::Path;

use bevy::{
    prelude::{Color as BevyColor, Image as BevyImage, *},
    render::{render_resource::SamplerDescriptor, texture::ImageSampler},
};
use image::{imageops, DynamicImage, ImageError, Rgba, RgbaImage};

use crate::prelude::Color;

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

    pub fn into_bevy_image(self) -> BevyImage {
        let mut image = BevyImage::from_dynamic(self.into_dyn(), true);
        image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
            mag_filter: bevy::render::render_resource::FilterMode::Nearest,
            min_filter: bevy::render::render_resource::FilterMode::Linear,
            ..default()
        });

        image
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
                .put_pixel(position.x, position.y, Rgba(color.as_rgba_u8()));
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

    pub fn get_pixel_vec(&self, position: Vec2) -> Result<Color, &str> {
        self.get_pixel_ivec(IVec2::new(
            position.x.round() as i32,
            position.y.round() as i32,
        ))
    }

    pub fn get_pixel_ivec(&self, position: IVec2) -> Result<Color, &str> {
        if position.x.is_negative() || position.y.is_negative() {
            Err("negative value")
        } else {
            self.get_pixel(position.as_uvec2())
        }
    }

    pub fn get_pixel(&self, position: UVec2) -> Result<Color, &str> {
        if self.contains_pixel(position) {
            let Rgba([r, g, b, a]) = *self.image.get_pixel(position.x, position.y);
            Ok(Color::from(BevyColor::rgba_u8(r, g, b, a)))
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
}
