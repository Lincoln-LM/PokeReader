use crate::pkrd::{display, display::Screen, reader, rng, views::view};
use ctr::res::CtrResult;

pub mod input {
    use ctr::hid::{Button, Global, InterfaceDevice};

    pub fn toggle() -> bool {
        Global::is_just_pressed(Button::Start | Button::Dup)
    }
}

pub fn draw(
    screen: &mut display::DirectWriteScreen,
    game: &impl reader::Gen2Reader,
    rng: &rng::Gen2Rng,
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let visual_frame = rng.get_visual_frame();
        let div_state = game.read_div();
        let rng_state = game.read_rng();

        view::draw_top_right(
            screen,
            "RNG Info",
            &[
                &alloc::format!("VFrame: {}", visual_frame),
                &alloc::format!("RNG: {:04X} rDIV: {:02X}", rng_state, div_state),
                &alloc::format!(
                    "{:03}:{:03}:{:03}",
                    game.read_gb::<u8>(0xD4B7),
                    game.read_gb::<u8>(0xD4B8),
                    game.read_gb::<u8>(0xD4B9)
                ),
                &alloc::format!(
                    "{:03}:{:03}:{:03}:{:03}",
                    ((game.read_gb::<u8>(0xD4C4) as u16) << 8)
                        | (game.read_gb::<u8>(0xD4C5) as u16),
                    game.read_gb::<u8>(0xD4C6),
                    game.read_gb::<u8>(0xD4C7),
                    game.read_gb::<u8>(0xD4C8),
                ),
            ],
        )?;
    }

    Ok(())
}
