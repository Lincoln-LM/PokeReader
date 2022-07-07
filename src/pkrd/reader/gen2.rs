use no_std_io::Reader;
use safe_transmute::TriviallyTransmutable;

#[cfg_attr(not(target_os = "horizon"), mocktopus::macros::mockable)]
pub trait Gen2Reader: Reader {
    const RAM_OFFSET: usize;
    const HRAM_OFFSET: usize;
    const VBLANK_OFFSET: usize;
    const DIV_OFFSET: usize;
    const RNG_OFFSET: usize;
    const TID_OFFSET: usize;
    const PARTY_OFFSET: usize;

    fn read_gb<T: TriviallyTransmutable + Default>(&self, offset: usize) -> T {
        self.default_read::<T>(if offset < 0xF000 {
            Self::RAM_OFFSET + offset
        } else {
            Self::HRAM_OFFSET + offset
        })
    }

    fn read_current_vblank(&self) -> u8 {
        self.read_gb(Self::VBLANK_OFFSET)
    }

    fn read_div(&self) -> u8 {
        self.read_gb(Self::DIV_OFFSET)
    }
    // idk why only reading u8 works here
    fn read_rng(&self) -> u16 {
        u16::from_le_bytes([
            self.read_gb(Self::RNG_OFFSET),
            self.read_gb(Self::RNG_OFFSET + 1),
        ])
    }

    fn read_tid(&self) -> u16 {
        u16::from_be_bytes([
            self.read_gb(Self::TID_OFFSET),
            self.read_gb(Self::TID_OFFSET + 1),
        ])
    }

    fn read_starter(&self) -> u16 {
        u16::from_le_bytes([
            self.read_gb(Self::PARTY_OFFSET),
            self.read_gb(Self::PARTY_OFFSET + 1),
        ])
    }
}
