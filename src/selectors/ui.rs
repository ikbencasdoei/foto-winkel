use bevy::prelude::*;
// use bevy_egui::{egui, EguiContext};

use crate::prelude::*;

pub struct SelectorCollectionPlugin;

impl Plugin for SelectorCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectorCollection>()
            .add_plugin(SelectorPlugin::<CanvasSelection>::default())
        // .add_system(ui)
        ;
    }
}

#[derive(PartialEq, Clone)]
pub struct SelectorIndex {
    pub name: String,
}

#[derive(Resource, Default)]
pub struct SelectorCollection {
    pub list: Vec<SelectorIndex>,
}

// fn ui(
//     mut egui_context: ResMut<EguiContext>,
//     collection: Res<SelectorCollection>,
//     mut editor: ResMut<Editor>,
// ) {
//     egui::Window::new("Selectors").show(egui_context.ctx_mut(), |ui| {
//         for selector in collection.list.iter() {
//             if ui
//                 .radio(
//                     editor.selected_selector == Some(selector.to_owned()),
//                     selector.name.to_owned(),
//                 )
//                 .clicked()
//             {
//                 if editor.selected_selector == Some(selector.to_owned()) {
//                     editor.selected_selector = None;
//                 } else {
//                     editor.selected_selector = Some(selector.clone());
//                 }
//             };
//         }
//     });
// }
