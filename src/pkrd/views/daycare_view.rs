use crate::pkrd::{display, display::Screen, reader};
use ctr::res::CtrResult;

pub fn run_daycare_view(
    game: &impl reader::Gen7Reader,
    screen: &mut display::DirectWriteScreen,
) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let mut x = 210;
        let mut y = 10;
        let black = display::Color::black();
        let white = display::Color::white();
        screen.paint_square(&black, x, y, 180, 72)?;
        x += 10;
        y += 4;
        screen.draw_string(&white, "Daycare View", x, y)?;
        y += 16;
        let egg_seed = game.get_egg_seed();
        screen.draw_string(&white, &alloc::format!("Egg[0]: {:08X}", egg_seed[3]), x, y)?;
        y += 12;
        screen.draw_string(&white, &alloc::format!("Egg[1]: {:08X}", egg_seed[2]), x, y)?;
        y += 12;
        screen.draw_string(&white, &alloc::format!("Egg[2]: {:08X}", egg_seed[1]), x, y)?;
        y += 12;
        screen.draw_string(&white, &alloc::format!("Egg[3]: {:08X}", egg_seed[0]), x, y)?;
    }

    Ok(())
}
