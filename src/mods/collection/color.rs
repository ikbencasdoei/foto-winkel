use bevy::prelude::*;
use bevy_egui::egui::{Color32, Ui};

use crate::prelude::{Color, Image, *};

#[derive(Clone, Default, PartialEq)]
pub struct ColorFilter {
    color: Color32,
}

impl Modifier for ColorFilter {
    fn apply(&mut self, mut input: Option<Image>, selection: Vec<UVec2>) -> Option<Image> {
        if let Some(image) = &mut input {
            for position in selection {
                image.set_pixel(position, Color::from(self.color)).ok();
            }
        }
        input
    }

    fn view(&mut self, ui: &mut Ui) {
        ui.label("color");
        ui.color_edit_button_srgba(&mut self.color);
    }
}