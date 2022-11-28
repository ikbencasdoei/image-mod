use std::path::Path;

use bevy::{
    prelude::{Color as BevyColor, Image as BevyImage, *},
    render::{render_resource::SamplerDescriptor, texture::ImageSampler},
};
use image::{DynamicImage, ImageError, Rgba, Rgba32FImage};

use crate::color::Color;

#[derive(Clone)]
pub struct Image {
    image: Rgba32FImage,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            image: Rgba32FImage::new(1, 1),
        }
    }
}

impl Image {
    pub fn from_dyn(image: DynamicImage) -> Self {
        Self {
            image: image.into_rgba32f(),
        }
    }

    pub fn into_dyn(self) -> DynamicImage {
        DynamicImage::ImageRgba32F(self.image)
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

    pub fn set_pixel(&mut self, position: UVec2, color: Color) -> Result<(), &str> {
        if self.contains_pixel(position) {
            self.image
                .put_pixel(position.x, position.y, Rgba(color.as_rgba_f32()));
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

    pub fn get_pixel(&mut self, position: UVec2) -> Result<Color, &str> {
        if self.contains_pixel(position) {
            let Rgba([r, g, b, a]) = self.image.get_pixel(position.x, position.y).clone();
            Ok(Color::from(BevyColor::rgba(r, g, b, a)))
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
}
