use super::reader;
use crate::pkrd::{
    command_handler, display,
    hook::{HookableProcess, HookedProcess, PatchPresentFramebufferConfig, SupportedTitle},
    rng, views,
};
use alloc::boxed::Box;
use ctr::{res::CtrResult, DebugProcess, Handle};

pub struct PokemonC {
    title: SupportedTitle,
    views: views::gen2::Gen2Views,
    rng: rng::Gen2Rng,
    reader: reader::PokemonCReader,
    command_handler: command_handler::CommandHandler,
}

impl HookedProcess for PokemonC {
    fn run_hook(
        &mut self,
        screen: &mut display::DirectWriteScreen,
        is_connected: bool,
    ) -> CtrResult<()> {
        self.views
            .run_views(screen, &self.reader, &mut self.rng, is_connected)?;
        if self.rng.get_visual_frame() == 1 && self.rng.get_vblank_change() != 0 {
            self.command_handler
                .clear_and_recieve_commands(is_connected);
        }
        self.command_handler
            .parse_command(&self.reader, self.rng.get_visual_frame());
        Ok(())
    }

    fn get_title(&self) -> SupportedTitle {
        self.title
    }
}

impl HookableProcess for PokemonC {
    fn new_from_supported_title(title: SupportedTitle, heap: &'static [u8]) -> Box<Self> {
        Box::new(Self {
            title,
            views: Default::default(),
            rng: Default::default(),
            reader: reader::PokemonCReader::new(heap),
            command_handler: Default::default(),
        })
    }

    fn install_hook(process: &DebugProcess, pkrd_handle: Handle) -> CtrResult<()> {
        let config = PatchPresentFramebufferConfig {
            is_extended_memory: false,
            get_screen_addr: 0x177ea4,
            present_framebuffer_addr: 0x14aa24,
            hook_vars_addr: 0x210000,
        };

        Self::patch_present_framebuffer(process, pkrd_handle, config)
    }
}
