use std::{convert::TryFrom, path::Path};

use bevy::{
    prelude::{Image as BevyImage, *},
    render::render_resource::TextureFormat,
};
use image::{DynamicImage, ImageBuffer};

#[derive(Deref, DerefMut)]
pub struct ImageHelper<'a> {
    image: &'a mut BevyImage,
}

const OFFSET: u32 = 4;

impl<'a> ImageHelper<'a> {
    pub fn new(image: &'a mut BevyImage) -> Self {
        Self { image }
    }

    fn get_index(&mut self, position: Vec2) -> Result<usize, &'static str> {
        let size = self.image.size().as_uvec2();

        let position = if position.x.is_sign_positive() && position.y.is_sign_positive() {
            position.as_uvec2()
        } else {
            return Err("negative position");
        };

        if position.x >= size.x || position.y >= size.y {
            return Err("position is outside image");
        }

        let i = ((size.x * position.y + position.x) * OFFSET) as usize;

        if i >= self.image.data.len() {
            return Err("position is outside image");
        }

        Ok(i)
    }

    pub fn set_pixel(&mut self, position: Vec2, color: Color) -> Result<(), &'static str> {
        match self.image.texture_descriptor.format {
            TextureFormat::Rgba8UnormSrgb => {
                let i = self.get_index(position)?;
                self.image
                    .data
                    .splice(i..(i + OFFSET as usize), color.as_rgba_u32().to_le_bytes());
                Ok(())
            }
            _ => Err("textureformat not supported"),
        }
    }

    pub fn get_pixel(&mut self, position: Vec2) -> Result<Color, &'static str> {
        let bytes: [u8; 4] = match self.image.texture_descriptor.format {
            TextureFormat::Rgba8UnormSrgb => {
                let i = self.get_index(position)?;
                self.image.data[i..(i + OFFSET as usize)]
                    .try_into()
                    .map_err(|_| "could not convert slice to array")?
            }
            _ => return Err("textureformat not supported"),
        };

        Ok(Color::rgba_u8(bytes[0], bytes[1], bytes[2], bytes[3]))
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), &'static str> {
        let image =
            DynamicImage::try_from(self.to_owned()).map_err(|_| "could not convert image")?;

        image.save(path).map_err(|_| "could not save image")
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
