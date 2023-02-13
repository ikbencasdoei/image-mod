use std::path::Path;

use image::{
    imageops::{self, FilterType},
    DynamicImage, ImageError, Rgba, RgbaImage,
};

use crate::{color::Color, position::Position};

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

    pub fn set_pixel(&mut self, position: Position, color: Color) -> Result<(), &str> {
        let (x, y) = position.try_into_u32()?;
        if self.contains_pixel(position) {
            self.image.put_pixel(x, y, Rgba(color.into_rgba_u8()));
            Ok(())
        } else {
            Err("pixel outside image")
        }
    }

    pub fn contains_pixel(&self, position: Position) -> bool {
        let (x, y) = if let Ok(position) = position.try_into_u32() {
            position
        } else {
            return false;
        };

        let (size_x, size_y) = self.size().try_into_u32().unwrap();
        (0..size_x).contains(&x) && (0..size_y).contains(&y)
    }

    pub fn size(&self) -> Position {
        let (x, y) = self.image.dimensions();
        Position::from_u32(x, y)
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = Position> {
        let (size_x, size_y) = self.size().into_i32();
        (0..(size_x * size_y)).map(move |a| Position::from_i32(a % size_x, a / size_x))
    }

    pub fn pixel_at(&self, position: Position) -> Result<Color, &str> {
        if self.contains_pixel(position) {
            let (x, y) = position.try_into_u32()?;
            let Rgba([r, g, b, a]) = *self.image.get_pixel(x, y);
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

    pub fn resize(&mut self, new_size: Position, filter: FilterType) -> Result<(), &str> {
        let (x, y) = new_size.try_into_u32()?;
        self.image = imageops::resize(&self.image, x, y, filter);
        Ok(())
    }

    pub fn overlay(&mut self, overlay: &Image, position: Position) {
        let (x, y) = position.into_i32();
        imageops::overlay(&mut self.image, &overlay.image, x.into(), y.into())
    }
}
