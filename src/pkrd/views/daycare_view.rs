use crate::pkrd::{display, display::Screen, reader};
use ctr::res::CtrResult;

pub fn run_daycare_view(
    game: &impl reader::Gen7Reader,
    screen: &mut display::DirectWriteScreen,
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let mut x = 220;
        let mut y = 10;

        let black = display::Color::black();
        let white = display::Color::white();

        screen.paint_square(&black, x, y, 170, 56)?;

        x += 10;
        y += 4;
        screen.draw_string(&white, "Hello from rust!", x, y)?;

        y += 12;
        screen.draw_string(&white, "Official Luma", x, y)?;

        y += 12;
        screen.draw_string(&white, "Not NTR", x, y)?;

        y += 16;
        let seed_text = &alloc::format!("Egg state[0]: {:08x}", 00000000);
        screen.draw_string(&white, seed_text, x, y)?;
    }

    Ok(())
}
