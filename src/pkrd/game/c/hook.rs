use crate::pkrd::{
    display,
    hook::{HookableProcess, HookedProcess, PatchPresentFramebufferConfig, SupportedTitle},
};
use alloc::boxed::Box;
use ctr::{res::CtrResult, DebugProcess, Handle};

pub struct PokemonC {
    title: SupportedTitle,
}

impl HookedProcess for PokemonC {
    fn run_hook(&mut self, _screen: &mut display::DirectWriteScreen) -> CtrResult<()> {
        Ok(())
    }

    fn get_title(&self) -> SupportedTitle {
        self.title
    }
}

impl HookableProcess for PokemonC {
    fn new_from_supported_title(title: SupportedTitle, _heap: &'static [u8]) -> Box<Self> {
        Box::new(Self { title })
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
