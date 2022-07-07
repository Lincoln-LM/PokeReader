use crate::pkrd::reader::Gen2Reader;
use no_std_io::Reader;

pub(super) struct PokemonCReader {
    heap: &'static [u8],
}

impl PokemonCReader {
    pub fn new(heap: &'static [u8]) -> Self {
        Self { heap }
    }
}

impl Reader for PokemonCReader {
    fn get_slice(&self) -> &[u8] {
        self.heap
    }
}

impl Gen2Reader for PokemonCReader {
    const RAM_OFFSET: usize = 0xA23FAC;
    const HRAM_OFFSET: usize = 0xA2C17C;
    const VBLANK_OFFSET: usize = 0xFF9B;
    const DIV_OFFSET: usize = 0xFF04;
    const RNG_OFFSET: usize = 0xFFE1;
    const TID_OFFSET: usize = 0xD47B;
    const PARTY_OFFSET: usize = 0xDCF4;
}
