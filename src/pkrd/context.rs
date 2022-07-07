use super::{
    display::{DirectWriteScreen, Screen},
    hook,
};
use alloc::boxed::Box;
use ctr::{res::CtrResult, sysmodule::server::ServiceContext};

pub struct PkrdServiceContext {
    pub screen: DirectWriteScreen,
    pub game: Option<Box<dyn hook::HookedProcess>>,
    pub is_paused: bool,
    pub is_connected: bool,
}

impl PkrdServiceContext {
    pub fn new() -> CtrResult<Self> {
        Ok(Self {
            screen: DirectWriteScreen::new(),
            game: None,
            is_paused: false,
            is_connected: false,
        })
    }
}

impl ServiceContext for PkrdServiceContext {}
