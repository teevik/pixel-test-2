use crate::game::constants::CHUNK_SIZE;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::ImageSampler;
use ndarray::Ix2;
use palette::Srgba;

pub struct ChunkTextureAccess<'a> {
    image: &'a mut Image,
}

impl<'a> ChunkTextureAccess<'a> {
    pub fn new(image: &'a mut Image) -> Self {
        Self { image }
    }

    pub fn set_color(&mut self, cell_index: Ix2, color: Srgba<u8>) {
        cell_index.
        let image_index_start = cell_index.into().ix() * 4;

        self.image.data[image_index_start] = color.red;
        self.image.data[image_index_start + 1] = color.green;
        self.image.data[image_index_start + 2] = color.blue;
        self.image.data[image_index_start + 3] = color.alpha;
    }

    pub fn set_colors(&mut self, cells: &[(Index, [u8; 4])]) {
        for (cell_index, color) in cells {
            let image_index_start = cell_index.0 * 4;

            self.image.data[image_index_start] = color[0];
            self.image.data[image_index_start + 1] = color[1];
            self.image.data[image_index_start + 2] = color[2];
            self.image.data[image_index_start + 3] = color[3];
        }
    }

    pub fn clear(&mut self) {
        for color_part in &mut self.image.data {
            *color_part = 0;
        }
    }
}

#[derive(Clone)]
pub struct ChunkTexture {
    image_handle: Handle<Image>,
}

impl ChunkTexture {
    pub fn new(images: &mut Assets<Image>) -> Self {
        let image = Image {
            sampler_descriptor: ImageSampler::nearest(),
            ..Image::new_fill(
                Extent3d {
                    width: CHUNK_SIZE as u32,
                    height: CHUNK_SIZE as u32,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                &[0, 0, 0, 0],
                TextureFormat::Rgba8UnormSrgb,
            )
        };

        let image_handle = images.add(image);

        Self { image_handle }
    }

    pub fn get_handle(&self) -> Handle<Image> {
        self.image_handle.clone()
    }

    pub fn access<'a>(&self, images: &'a mut Assets<Image>) -> ChunkTextureAccess<'a> {
        let image = images.get_mut(&self.image_handle).unwrap();

        ChunkTextureAccess::<'a>::new(image)
    }
}
