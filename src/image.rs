use std::path::Path;

use bevy::{
    prelude::{Image, Vec2},
    render::render_resource::TextureFormat,
};
use image::{DynamicImage, ImageBuffer};

pub struct ImageHelper<'a> {
    image: &'a mut Image,
}

impl<'a> ImageHelper<'a> {
    pub fn new(image: &'a mut Image) -> Self {
        Self { image }
    }

    pub fn set_pixel(&mut self, position: Vec2, color: [u8; 4]) {
        let size = self.image.size();

        if position.x < 0. || position.x > size.x || position.y < 0. || position.y > size.y {
            return;
        }

        let (size, position) = (size.as_uvec2(), position.as_uvec2());

        match self.image.texture_descriptor.format {
            TextureFormat::Rgba8UnormSrgb => {
                let offset = 4;
                let i = ((size.x * (position.y) + position.x) * offset) as usize;
                self.image.data.splice(i..(i + offset as usize), color);
            }
            _ => todo!(),
        }
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), ()> {
        let image = DynamicImage::try_from(self.to_owned())?;

        image.save(path).map_err(|_| ())
    }
}

impl TryFrom<&ImageHelper<'_>> for DynamicImage {
    type Error = ();

    fn try_from(helper: &ImageHelper<'_>) -> Result<Self, Self::Error> {
        let image = &helper.image;
        let option = match image.texture_descriptor.format {
            TextureFormat::R8Unorm => ImageBuffer::from_raw(
                image.texture_descriptor.size.width,
                image.texture_descriptor.size.height,
                image.data.clone(),
            )
            .map(DynamicImage::ImageLuma8),
            TextureFormat::Rg8Unorm => ImageBuffer::from_raw(
                image.texture_descriptor.size.width,
                image.texture_descriptor.size.height,
                image.data.clone(),
            )
            .map(DynamicImage::ImageLumaA8),
            TextureFormat::Rgba8UnormSrgb => ImageBuffer::from_raw(
                image.texture_descriptor.size.width,
                image.texture_descriptor.size.height,
                image.data.clone(),
            )
            .map(DynamicImage::ImageRgba8),
            _ => None,
        };

        match option {
            Some(image) => Ok(image),
            None => Err(()),
        }
    }
}
