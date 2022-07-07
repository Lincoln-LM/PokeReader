use crate::{
    pkrd::{display, display::Screen, views::view},
    CONNE_SOCKET, LOCAL_SOCKET,
};
use ctr::res::CtrResult;

pub mod input {
    use ctr::hid::{Button, Global, InterfaceDevice};

    pub fn toggle() -> bool {
        Global::is_just_pressed(Button::Start | Button::Ddown)
    }
}

pub fn draw(screen: &mut display::DirectWriteScreen, is_connected: bool) -> CtrResult<()> {
    if screen.get_is_top_screen() {
        let local_addr = LOCAL_SOCKET.lock();
        let remote_addr = CONNE_SOCKET.lock();
        let local_addr = alloc::format!(
            "IP: {}.{}.{}.{}:{}",
            local_addr[0],
            local_addr[1],
            local_addr[2],
            local_addr[3],
            7000
        );
        let remote_addr = alloc::format!(
            "CN: {}.{}.{}.{}:{}",
            remote_addr[0],
            remote_addr[1],
            remote_addr[2],
            remote_addr[3],
            remote_addr[4]
        );
        view::draw_top_left(
            screen,
            "Network Info",
            &[
                &local_addr,
                if !is_connected {
                    "L+R+Select to connect"
                } else {
                    &remote_addr
                },
            ],
        )?;
    }

    Ok(())
}
