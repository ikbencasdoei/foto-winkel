#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(option_result_contains)]

use std::path::Path;

use bevy::prelude::{Color, *};
use bevy_egui::{EguiPlugin, EguiSettings};
use editor::{Editor, EditorPlugin};
use file_picker::FilePickerPlugin;
use keybinds::KeyBindsPlugin;
use mods::ui::ModifierUiPlugin;
use ui::UiPlugin;
use view::ViewPlugin;

mod color;
mod editor;
mod file_picker;
mod image;
mod keybinds;
mod mods;
mod ui;
mod view;

fn main() {
    dotenvy::dotenv().ok();

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                fit_canvas_to_parent: true,
                title: env!("CARGO_PKG_NAME").to_string(),
                ..default()
            },
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(FilePickerPlugin)
        .add_plugin(ViewPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(KeyBindsPlugin)
        .add_plugin(ModifierUiPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut egui_settings: ResMut<EguiSettings>, mut editor: ResMut<Editor>) {
    egui_settings.scale_factor = 1.5;

    if let Ok(path) = std::env::var("NEW_PROJECT_INPUT_PATH") {
        *editor = Editor::new_from_input_path(Path::new(&path))
    }
}
