use crate::pkrd::reader::Gen7Reader;
use no_std_io::Reader;

pub(super) struct PokemonSMReader {
    heap: &'static [u8],
}

impl PokemonSMReader {
    pub fn new(heap: &'static [u8]) -> Self {
        Self { heap }
    }
}

impl Reader for PokemonSMReader {
    fn get_slice(&self) -> &[u8] {
        self.heap
    }
}

impl Gen7Reader for PokemonSMReader {
    const INITIAL_SEED_OFFSET: usize = 0x25A3878;
    const SFMT_STATE_INDEX_OFFSET: usize = 0x3196548;
    const SFMT_STATE_OFFSET: usize = 0x3195B88;
    const PARTY_OFFSET: usize = 0x4195E10;
    const WILD_OFFSET: usize = 0x002F7B8;
    const SOS_OFFSET: usize = 0x002F7B8;
    const SOS_SEED_OFFSET: usize = 0x0038C44;
    const SOS_CHAIN_LENGTH: usize = 0x003960D;
    const EGG_READY_OFFSET: usize = 0x313EDD8;
    const EGG_OFFSET: usize = 0x313EDDC;
    const PARENT1_OFFSET: usize = 0x313EC01;
    const PARENT2_OFFSET: usize = 0x313ECEA;
    const IS_PARENT1_OCCUPIED_OFFSET: usize = 0x313EC00;
    const IS_PARENT2_OCCUPIED_OFFSET: usize = 0x313ECE9;
    const SHINY_CHARM_OFFSET: usize = 0x30d5930;
}
