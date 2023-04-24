use crate::game::data::behaviour::Behaviour;
use palette::Srgba;

pub struct Cell {
    last_updated_at: u64,
    color: Srgba<u8>,
    behaviour: Behaviour,
}
