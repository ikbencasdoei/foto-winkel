use std::{convert::TryFrom, path::Path};

use bevy::{
    prelude::{Color as BevyColor, Image as BevyImage, *},
    render::render_resource::TextureFormat,
};
use image::{DynamicImage, Rgba, Rgba32FImage};

use crate::color::Color;

pub struct Image {
    image: Rgba32FImage,
}

impl Image {
    pub fn from_dyn(image: DynamicImage) -> Self {
        Self {
            image: image.into_rgba32f(),
        }
    }

    pub fn into_dyn(self) {
        DynamicImage::ImageRgba32F(self.image);
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
}
