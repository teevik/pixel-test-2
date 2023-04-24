use crate::game::components::{MainCamera, PixelSimulation};
use crate::game::constants::WORLD_CHUNK_SIZE;
use bevy::prelude::*;
use crate::game::data::loaded_chunks::LoadedChunks;

pub fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Name::new("Camera"), MainCamera, Camera2dBundle::default()));

    commands.spawn(UiCameraConfig::default());
    
    let mut chunks = LoadedChunks::new();
    
    

    // .insert(Name::new("Camera"))
    // .insert()
    // .insert_bundle(OrthographicCameraBundle {
    //     transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)),
    //     ..OrthographicCameraBundle::new_2d()
    // });

    // let mut chunks = LoadedChunks::new(&mut images);
    //
    let mut children = Vec::<Entity>::new();
    
    for (chunk_index, chunk, texture) in &chunks.chunks {
        let chunk_position = chunk_index.as_point();
    
        let child = commands
            .spawn((
                Name::new(format!("Chunk {} {}", chunk_position.x, chunk_position.y)),
                SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                    (chunk_position.x as f32 - 1.) * WORLD_CHUNK_SIZE,
                    -(chunk_position.y as f32 - 1.) * WORLD_CHUNK_SIZE,
                    0.,
                ))),
            ))
            .with_children(|child_builder| {
                child_builder.spawn(SpriteBundle {
                    texture: texture.get_handle(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::ONE * WORLD_CHUNK_SIZE),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            })
            .id();
    
        children.push(child);
    }
    
    commands
        .spawn((
            Name::new("Pixel Simulation"),
            PixelSimulation { chunks },
            SpatialBundle::default(),
        ))
        .push_children(&children);
}
