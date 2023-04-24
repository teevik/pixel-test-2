use crate::game::data::loaded_chunks::LoadedChunks;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct PixelSimulation {
    pub chunks: LoadedChunks,
}
