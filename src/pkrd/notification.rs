use super::hook;
use crate::log;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use ctr::{ptm, res::CtrResult, sysmodule::notification::NotificationHandlerResult};

static IS_NEW_GAME_LAUNCH: AtomicBool = AtomicBool::new(false);
// store this because VC has issues, implement AtomicU64?
pub static GAME_TITLE_ID_HIGH: AtomicU32 = AtomicU32::new(0);
pub static GAME_TITLE_ID_LOW: AtomicU32 = AtomicU32::new(0);

/// Determines if a game was just launched.
/// After this has been called once, it will always return `false` until a game is launched.
pub fn is_new_game_launch() -> bool {
    IS_NEW_GAME_LAUNCH.swap(false, Ordering::Relaxed)
}

pub fn handle_launch_title_notification(_notification_id: u32) -> CtrResult<()> {
    IS_NEW_GAME_LAUNCH.store(true, Ordering::Relaxed);
    if let Some(title) = hook::SupportedTitle::from_running_app() {
        let hook_result = hook::install_hook(title);

        if hook_result.is_err() {
            log::error(&alloc::format!(
                "Failed to hook title {:x}",
                u64::from(title)
            ));
        } else {
            GAME_TITLE_ID_HIGH.store(((title as u64) >> 32u64) as u32, Ordering::Relaxed);
            GAME_TITLE_ID_LOW.store(((title as u64) & 0xFFFFFFFFu64) as u32, Ordering::Relaxed);
        }

        return hook_result;
    }

    Ok(())
}

/// The notification Id is currently a u32 to avoid assumptions about the notifications that might be sent.
///
/// However it's probably safe to assume only [0x100, 0x179](https://github.com/LumaTeam/Luma3DS/blob/ebeef7ab7f730ae35658b66ca97c5da9f663a17d/sysmodules/loader/source/service_manager.c#L58-L59), and subscribed notifications will be used here, so an enum may be better here in the future.
pub fn handle_sleep_notification(notification_id: u32) -> NotificationHandlerResult {
    ptm::sysm_init()?;

    if notification_id == ptm::NotificationId::SleepRequested {
        // Sleeping and logging seem to interfere with each other,
        // so we deny sleeping when in dev mode
        #[cfg(debug_assertions)]
        ptm::sys_reply_to_sleep_query(true)?;

        #[cfg(not(debug_assertions))]
        ptm::sys_reply_to_sleep_query(false)?;
    } else {
        let ack_value = ptm::sys_get_notification_ack_value(notification_id);
        ptm::sys_notify_sleep_preparation_complete(ack_value)?;
    }

    ptm::sysm_exit();
    Ok(())
}
