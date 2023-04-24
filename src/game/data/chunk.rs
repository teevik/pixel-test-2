use crate::game::constants::CHUNK_SIZE;
use crate::game::data::cell::Cell;
use ndarray::Array2;

pub struct Chunk {
    cells: Array2<Option<Cell>>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            cells: Array2::default((CHUNK_SIZE, CHUNK_SIZE)),
        }
    }
}
