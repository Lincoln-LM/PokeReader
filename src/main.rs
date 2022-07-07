#![no_std]
#![allow(incomplete_features)]
#![feature(alloc_error_handler)]
#![feature(start)]
#![feature(if_let_guard)]
#![allow(dead_code)]

extern crate alloc;

#[doc(hidden)]
mod heap_allocator;
mod log;
mod pkrd;
mod utils;

use crate::pkrd::{
    context::PkrdServiceContext,
    notification::{handle_launch_title_notification, handle_sleep_notification},
    request_handler::handle_pkrd_game_request,
};
use alloc::{boxed::Box, vec};
use core::convert::TryFrom;
#[cfg(not(test))]
use core::{arch::asm, panic::PanicInfo};
use ctr::{
    fs, memory, ptm, socket, srv, svc,
    sysmodule::{
        notification::NotificationManager,
        server::{Service, ServiceManager},
    },
};
use lazy_static::lazy_static;
use spin::Mutex;

/// Called after main exits to clean things up.
/// Used by 3ds toolchain.
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __wrap_exit() {
    svc::exit_process();
}

#[repr(align(0x1000))]
struct SocketBuffer([u8; 0x100000]);

impl SocketBuffer {
    fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

static mut SOCKET_BUFFER: SocketBuffer = SocketBuffer([0; 0x100000]);

lazy_static! {
    static ref SOCKET: Mutex<socket::SocketContext> =
        Mutex::new(socket::SocketContext::new(socket::SocketType::Stream).unwrap());
    static ref LOCAL_SOCKET: Mutex<[u8; 4]> = Mutex::new([0u8; 4]);
    static ref CONNE_SOCKET: Mutex<[u16; 5]> = Mutex::new([0u16; 5]);
}

/// Called before main to initialize the system.
/// Used by 3ds toolchain.
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn initSystem() {
    // This is safe because we're only supposed to use this one time
    // while initializing the system, which is happening right here.
    unsafe { heap_allocator::init_heap() };

    loop {
        match srv::init() {
            Ok(_) => break,
            Err(error_code) => {
                if error_code != 0xd88007fa {
                    panic!();
                }
            }
        };

        svc::sleep_thread(500000i64);
    }

    fs::init().unwrap();

    // This is safe as long as we're single threaded
    let aligned_buffer = unsafe { SOCKET_BUFFER.as_mut_slice() };
    let memory_block = memory::MemoryBlock::new(
        aligned_buffer,
        memory::MemoryPermission::None,
        memory::MemoryPermission::ReadWrite,
    )
    .unwrap();
    socket::socu_init(memory_block).expect("soc:U did not init");
}

#[cfg(not(test))]
#[doc(hidden)]
#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    if let Some(location) = panic.location() {
        let file = location.file();
        let slice = &file[file.len() - 7..];

        // Since we're about to break, storing a few u32s in these registers won't break us further.
        // In the future it might be helpful to disable this for release builds.
        unsafe {
            // r9 and r10 aren't used as frequently as the lower registers, so in most situations
            // we'll get more useful information by storing the last 4 characters of the file name
            // and the line number where we broke.
            let partial_file_name = *(slice.as_ptr() as *const u32);
            asm!("mov r9, {}", in(reg) partial_file_name);
            asm!("mov r10, {}", in(reg) location.line());
        }
    }

    ctr::svc::break_execution(ctr::svc::UserBreakType::Panic)
}

#[cfg(not(test))]
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn abort() -> ! {
    ctr::svc::break_execution(ctr::svc::UserBreakType::Panic)
}

#[doc(hidden)]
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    log::debug("\n\nStarted!");
    // let mut tcp_socket = socket::SocketContext::new(socket::SocketType::Stream).unwrap();
    {
        let tcp_socket = SOCKET.lock();
        tcp_socket
            .bind(socket::SocketAddr::new(
                tcp_socket.get_host_id().unwrap(),
                7000,
            ))
            .expect("");
        tcp_socket.listen(5).expect("");
        LOCAL_SOCKET
            .lock()
            .clone_from_slice(&tcp_socket.get_host_id().unwrap());
    };
    // tcp_socket.bind(socket::SocketAddr::new(tcp_socket.get_host_id().unwrap(), 7000)).expect("");
    // tcp_socket.listen(5).expect("");
    // tcp_socket.accept().expect("");
    let global_context = Box::new(PkrdServiceContext::new().unwrap());

    let services = vec![Service::new("pkrd:game", 8, handle_pkrd_game_request).unwrap()];

    log::debug("Setting up notification manager");

    let mut notification_manger = NotificationManager::new().unwrap();

    notification_manger
        .subscribe(
            ptm::NotificationId::SleepRequested,
            handle_sleep_notification,
        )
        .unwrap();
    notification_manger
        .subscribe(ptm::NotificationId::GoingToSleep, handle_sleep_notification)
        .unwrap();
    notification_manger
        .subscribe(
            ptm::NotificationId::FullyWakingUp,
            handle_sleep_notification,
        )
        .unwrap();
    notification_manger
        .subscribe(
            ptm::NotificationId::LaunchApp,
            handle_launch_title_notification,
        )
        .unwrap();

    log::debug("Setting up service manager");
    let mut manager = ServiceManager::new(services, notification_manger, global_context);
    log::debug("Set up service manager");
    let result = manager.run();

    match result {
        Ok(_) => 0,
        Err(result_code) => {
            let raw_code = result_code.into_raw();
            log::error(&alloc::format!("manager.run error {:x}", raw_code));
            isize::try_from(raw_code).unwrap()
        }
    }
}
