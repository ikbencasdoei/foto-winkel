use std::path::PathBuf;

use bevy::{
    prelude::{Image as BevyImage, *},
    render::{render_resource::SamplerDescriptor, texture::ImageSampler},
};

use crate::{
    image::ImageHelper,
    tools::{CurrentTool, ToolEvent},
};

mod ui;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ui::Plugin)
            .add_system(events)
            .add_system(set_filter);
    }
}

#[derive(Component, Default)]
pub struct Sprite {
    pub image: Option<Handle<BevyImage>>,
    pub image_path: Option<PathBuf>,
    pub target_scale: Option<Vec3>,
    pub target_translation: Option<Vec3>,
}

fn events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<ui::Event>,
    mut entity: Query<(Entity, &mut Sprite)>,
    mut assets: ResMut<Assets<BevyImage>>,
    mut event_writer: EventWriter<ToolEvent>,
    current_tool: Res<CurrentTool>,
) {
    for event in event_reader.iter() {
        match event {
            ui::Event::PickerOpened => (),
            ui::Event::PickedOpen(path) => {
                for (entity, _) in entity.iter() {
                    commands.entity(entity).despawn();
                }

                asset_server.reload_asset(path.to_owned());

                let handle = asset_server.load(path.to_owned());

                commands
                    .spawn_bundle(SpriteBundle {
                        texture: handle.clone(),
                        ..default()
                    })
                    .insert(Sprite {
                        image: Some(handle),
                        image_path: Some(path.to_owned()),
                        ..default()
                    });

                event_writer.send(ToolEvent::Switched {
                    from: current_tool.to_owned(),
                    to: current_tool.to_owned(),
                })
            }
            ui::Event::PickedSave(path) => {
                for (_, mut sprite) in entity.iter_mut() {
                    let image =
                        ImageHelper::new(assets.get_mut(sprite.image.as_ref().unwrap()).unwrap());

                    image.save(path).unwrap();

                    sprite.image_path = Some(path.to_owned());
                }
            }
            _ => (),
        }
    }
}

fn set_filter(
    mut events: EventReader<AssetEvent<BevyImage>>,
    mut assets: ResMut<Assets<BevyImage>>,
    query: Query<&Sprite>,
) {
    for event in events.iter() {
        if let AssetEvent::Created { handle } = event {
            for sprite in query.iter() {
                if let Some(image) = &sprite.image {
                    if image.id == handle.id {
                        if let Some(mut image) = assets.get_mut(handle) {
                            image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
                                mag_filter: bevy::render::render_resource::FilterMode::Nearest,
                                min_filter: bevy::render::render_resource::FilterMode::Linear,
                                ..default()
                            })
                        }
                    }
                }
            }
        }
    }
}
