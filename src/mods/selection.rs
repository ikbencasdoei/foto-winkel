use bevy::prelude::*;

use crate::image::Image;

pub trait Selection {
    fn get_pixels(&self, image: &Image) -> Vec<UVec2>;
}

pub struct CanvasSelection;

impl Selection for CanvasSelection {
    fn get_pixels(&self, image: &Image) -> Vec<UVec2> {
        let size = image.size();

        (0..(size.x * size.y))
            .map(|i| UVec2::new(i % size.x, i / size.x))
            .collect()
    }
}