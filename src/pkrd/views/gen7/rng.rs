use crate::pkrd::{display, reader, reader::RngSlot, rng, views::view};
use ctr::res::CtrResult;

pub mod input {
    use super::*;
    use ctr::hid::{Button, Global, InterfaceDevice};

    pub fn toggle() -> bool {
        Global::is_just_pressed(Button::Start | Button::Dup)
    }

    fn increment() -> bool {
        Global::is_just_pressed(Button::Select | Button::Dright)
    }

    fn decrement() -> bool {
        Global::is_just_pressed(Button::Select | Button::Dleft)
    }

    pub fn next_rng_slot(mut slot: RngSlot) -> RngSlot {
        if increment() {
            slot.increment();
        }

        if decrement() {
            slot.decrement();
        }

        slot
    }
}

pub fn draw(
    screen: &mut display::DirectWriteScreen,
    game: &impl reader::Gen7Reader,
    rng: &rng::Gen7Rng,
    rng_slot: RngSlot,
) -> CtrResult<()> {
    if rng_slot.value() == 0 {
        draw_main(screen, game, rng)?;
    } else {
        draw_sos(screen, game)?;
    }

    Ok(())
}

pub fn draw_main(
    screen: &mut display::DirectWriteScreen,
    game: &impl reader::Gen7Reader,
    rng: &rng::Gen7Rng,
) -> CtrResult<()> {
    let init_seed = game.get_initial_seed();
    let sfmt_state = game.get_sfmt_state();
    let sfmt_advances = rng.get_sfmt_advances();
    let vframe = rng.get_vframe();
    let tid = game.get_tid();
    let tsv = game.get_tsv();

    view::draw_top_right(
        screen,
        "Main RNG View",
        &[
            &alloc::format!("Init seed: {:08X}", init_seed),
            &alloc::format!("Curr state[1]: {:08X}", (sfmt_state & 0xffffffff) as u32),
            &alloc::format!("Curr state[0]: {:08X}", (sfmt_state >> 32) as u32),
            &alloc::format!("Advances: {}", sfmt_advances),
            &alloc::format!("VFrame: {}", vframe),
            &alloc::format!("Gen7TID: {}", tid),
            &alloc::format!("TSV: {}", tsv),
        ],
    )
}

pub fn draw_sos(
    screen: &mut display::DirectWriteScreen,
    game: &impl reader::Gen7Reader,
) -> CtrResult<()> {
    let sos_seed = game.get_sos_seed();
    let sos_chain = game.get_sos_chain();

    view::draw_top_right(
        screen,
        "SOS RNG View",
        &[
            &alloc::format!("SOS Seed: {:08X}", sos_seed),
            &alloc::format!("SOS Chain Length: {}", sos_chain),
        ],
    )
}
