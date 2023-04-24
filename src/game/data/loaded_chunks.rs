use crate::game::data::chunk::Chunk;
use ndarray::Array2;

pub struct LoadedChunks {
    pub chunks: Array2<Chunk>,
}

impl LoadedChunks {
    pub fn new() -> Self {
        Self {
            chunks: Array2::from_shape_fn((3, 3), |(x, y)| Chunk::new()),
        }
    }
    
    pub fn a(self) {
       self.chunks.get() 
    }
}
