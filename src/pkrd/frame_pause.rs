use crate::{CONNE_SOCKET, SOCKET};

use super::context::PkrdServiceContext;
use ctr::{hid, hid::InterfaceDevice, svc};

pub fn handle_frame_pause(context: &mut PkrdServiceContext, is_top_screen: bool) {
    if hid::Global::is_just_pressed(hid::Button::R) {
        context.is_paused = true;
    }
    if !context.is_connected
        && hid::Global::is_just_pressed(
            hid::Button::L | hid::Button::R | hid::Button::Select as u32,
        )
    {
        let connected_address = SOCKET.lock().accept().unwrap();
        let mut socket_addr = CONNE_SOCKET.lock();
        socket_addr[0] = connected_address.address[0] as u16;
        socket_addr[1] = connected_address.address[1] as u16;
        socket_addr[2] = connected_address.address[2] as u16;
        socket_addr[3] = connected_address.address[3] as u16;
        socket_addr[4] = connected_address.port;

        context.is_connected = true;
    }

    while context.is_paused && is_top_screen {
        hid::Global::scan_input();

        let just_down = hid::Global::just_down_buttons();
        let held_buttons = hid::Global::held_buttons();

        if held_buttons.l() && just_down.a() {
            break;
        }

        if just_down.l() {
            break;
        }

        if just_down.a() || just_down.r() {
            context.is_paused = false;
            break;
        }

        svc::sleep_thread(50000000);
    }
}
