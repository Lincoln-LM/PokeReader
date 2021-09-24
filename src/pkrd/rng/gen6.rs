use super::mt;
use super::tinymt;
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Gen6Rng {
    init_seed: u32,
    init_tinymt_state: [u32; 4],
    mt_rng: mt::MT,
    tinymt_rng: tinymt::TinyMT,
    mt_advances: u32,
    tinymt_advances: u32,
    mt_state: u32,
}

impl Gen6Rng {
    pub fn get_mt_advances(&self) -> u32 {
        self.mt_advances
    }

    pub fn get_tinymt_advances(&self) -> u32 {
        self.tinymt_advances
    }

    pub fn get_initial_tinymt_state(&self) -> [u32; 4] {
        self.init_tinymt_state
    }

    pub fn get_tinymt_state(&self) -> [u32; 4] {
        self.tinymt_rng.get_state()
    }

    pub fn get_mt_state(&self) -> u32 {
        self.mt_state
    }

    pub fn get_patches(&mut self) -> [[u8;9];9] {
        let mut patches = [[5;9];9];
        patches[4][4] = 4;
        let mut go = self.tinymt_rng;
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();
        go.next_state();

        go.next_state();
        let good_rate = [23,43,63,83];
        let mut ring = 0u64;
        while ring < 4 {
            let mut state = 0;
            let mut direction = ((go.next() as u64 * 4u64) >> 32);
            let mut location = ((go.next() as u64 * (ring as u64 * 2u64 + 3u64)) >> 32);
            if ((go.next() as u64 * 100) >> 32) < good_rate[ring as usize] {
                state = 1;
                go.next();
                go.next();
            }
            patches = self.set_patch(patches,ring as u32,direction as u32,location as u32,state);
            ring += 1;
        }
        ring = ((go.next() as u64 * 3u64) >> 32);
        let mut direction = ((go.next() as u64 * 4u64) >> 32);
        let mut location = ((go.next() as u64 * (ring as u64 * 2u64 + 3u64)) >> 32);
        let mut state = 3;
        patches = self.set_patch(patches,ring as u32,direction as u32,location as u32,state);
        patches
    }

    pub fn set_patch(&mut self, mut patches: [[u8;9];9], ring: u32, direction: u32, location: u32, state: u8) -> [[u8;9];9] {
        let mut x = 4;
        let mut y = 4;
        if direction == 0 || direction == 1 {
            x = 3 - ring + location;
        }
        else if direction == 2 {
            x = 3 - ring;
        }
        else {
            x = 5 + ring;
        }
        if direction == 0 {
            y = 3 - ring;
        }
        else if direction == 1 {
            y = 5 + ring;
        }
        else {
            y = 3 - ring + location;
        }
        patches[x as usize][y as usize] = state;
        patches
    }

    pub fn update(&mut self, mt_state: u32, init_seed: u32, tinymt_state: [u32; 4]) {
        if self.init_seed != init_seed && init_seed != 0 {
            self.mt_rng = mt::MT::new(init_seed);
            self.tinymt_rng = tinymt::TinyMT::new(tinymt_state);
            self.mt_advances = 0;
            self.tinymt_advances = 0;
            self.init_seed = init_seed;
            self.init_tinymt_state = tinymt_state;
            self.mt_state = init_seed;
        }

        // A boundary of 9999 makes sure we can't go in an infinite loop
        let mut temp_mt_state = self.mt_state;
        for advances in 0..9999 {
            if mt_state == temp_mt_state || mt_state == 0 {
                self.mt_state = temp_mt_state;
                self.mt_advances += advances;
                break;
            }
            temp_mt_state = self.mt_rng.next();
        }

        // Same as above, 9999 prevents infinite loop
        for tinymt_advances in 0..9999 {
            if tinymt_state == self.get_tinymt_state() || tinymt_state == [0, 0, 0, 0] {
                self.tinymt_advances += tinymt_advances;
                break;
            }
            self.tinymt_rng.next_state();
        }
    }
}
