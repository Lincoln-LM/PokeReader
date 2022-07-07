use crate::pkrd::reader;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Gen2Rng {
    last_vblank: u8,
    vblank_change: u8,
    visual_frame: u32,
}

impl Gen2Rng {
    pub fn get_visual_frame(&self) -> u32 {
        self.visual_frame
    }

    pub fn get_last_vblank(&self) -> u8 {
        self.last_vblank
    }

    pub fn get_vblank_change(&self) -> u8 {
        self.vblank_change
    }

    fn update_visual_frame(&mut self, current_vblank: u8) {
        self.vblank_change = current_vblank.wrapping_sub(self.last_vblank);
        self.visual_frame += self.vblank_change as u32;
        self.last_vblank = current_vblank;
    }

    pub fn update(&mut self, game: &impl reader::Gen2Reader) {
        let current_vblank = game.read_current_vblank();
        self.update_visual_frame(current_vblank);
        // tiny chance we run into this naturally
        if current_vblank == 0 && game.read_rng() == 0x0000 {
            self.visual_frame = 0
        }
    }
}

// TODO:
// #[cfg(test)]
// mod test {
//     use super::*;
//     use mocktopus::mocking::{MockResult, Mockable};
//     use no_std_io::Reader;
// }
