use super::network as network_view;
use super::rng as rng_view;
use crate::pkrd::{display, reader, rng};
use ctr::res::CtrResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LeftGen2View {
    None,
    NetworkView,
    // PartyView,
    // WildView,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RightGen2View {
    None,
    RngView,
    // DaycareView,
}

pub struct Gen2Views {
    left_view: LeftGen2View,
    right_view: RightGen2View,
    // party_slot: PartySlot,
    // daycare_slot: DaycareSlot,
}

impl Default for Gen2Views {
    fn default() -> Self {
        Self {
            left_view: LeftGen2View::None,
            right_view: RightGen2View::None,
            // party_slot: PartySlot::default(),
            // daycare_slot: DaycareSlot::default(),
        }
    }
}

impl Gen2Views {
    fn update_views(&mut self) {
        self.right_view = match self.right_view {
            RightGen2View::RngView if rng_view::input::toggle() => RightGen2View::None,
            _ if rng_view::input::toggle() => RightGen2View::RngView,
            view => view,
        };

        self.left_view = match self.left_view {
            LeftGen2View::NetworkView if network_view::input::toggle() => LeftGen2View::None,
            _ if network_view::input::toggle() => LeftGen2View::NetworkView,
            view => view,
        };
    }

    pub fn run_views<GameReader: reader::Gen2Reader>(
        &mut self,
        screen: &mut display::DirectWriteScreen,
        game: &GameReader,
        rng: &mut rng::Gen2Rng,
        is_connected: bool,
    ) -> CtrResult<()> {
        rng.update(game);
        self.update_views();

        match self.left_view {
            LeftGen2View::NetworkView => network_view::draw(screen, is_connected)?,
            LeftGen2View::None => {}
        }

        match self.right_view {
            RightGen2View::RngView => rng_view::draw(screen, game, rng)?,
            RightGen2View::None => {}
        }

        Ok(())
    }
}
