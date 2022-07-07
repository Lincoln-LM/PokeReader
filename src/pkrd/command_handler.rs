use core::convert::{TryFrom, TryInto};

use alloc::vec::Vec;
use ctr::{os::get_time, utils::convert::try_usize_into_u32};

use crate::SOCKET;

use super::reader;

#[repr(u32)]
#[derive(PartialEq, Eq)]
pub enum Command {
    End = 0,
    Input = 1,
    ImpreciseInput = 2,
    Read = 3,

    WaitUntil = 255,
}

impl TryFrom<u32> for Command {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            v if v == Command::End as u32 => Ok(Command::End),
            v if v == Command::Input as u32 => Ok(Command::Input),
            v if v == Command::ImpreciseInput as u32 => Ok(Command::ImpreciseInput),
            v if v == Command::Read as u32 => Ok(Command::Read),
            v if v == Command::WaitUntil as u32 => Ok(Command::WaitUntil),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CommandHandler {
    imprecise_input_storage: [u64; 2],
    inprecise_input_started: bool,
    input_started: bool,
    commands: Vec<[u32; 4]>,
    command_index: usize,
    last_command_ended: u32,
}

impl CommandHandler {
    pub fn clear_and_recieve_commands(&mut self, is_connected: bool) {
        self.commands = Vec::<[u32; 4]>::new();
        self.command_index = 0;
        self.last_command_ended = 0;
        self.imprecise_input_storage = [0u64; 2];
        self.inprecise_input_started = false;
        self.input_started = false;
        let mut input_array = [0u8; 1024];
        if is_connected {
            let tcp_socket = SOCKET.lock();
            if tcp_socket.recv(&mut input_array, 0).is_ok() {
                let mut command_length = 64;
                for i in 0..64 {
                    let command = [
                        u32::from_le_bytes(input_array[i * 16..i * 16 + 4].try_into().unwrap()),
                        u32::from_le_bytes(input_array[i * 16 + 4..i * 16 + 8].try_into().unwrap()),
                        u32::from_le_bytes(
                            input_array[i * 16 + 8..i * 16 + 12].try_into().unwrap(),
                        ),
                        u32::from_le_bytes(
                            input_array[i * 16 + 12..i * 16 + 16].try_into().unwrap(),
                        ),
                    ];
                    let command_type: Command = command[0].try_into().unwrap();
                    if command_type == Command::End {
                        command_length = i;
                        break;
                    }
                    self.commands.push(command);
                }
                let command_count_buffer =
                    try_usize_into_u32(command_length).unwrap().to_le_bytes();
                let _ = tcp_socket.send(&command_count_buffer, 0);
            }
        }
    }

    pub fn parse_command(&mut self, game: &impl reader::Gen2Reader, visual_frame: u32) {
        if self.command_index < self.commands.len() {
            let command = self.commands[self.command_index];
            let command_type = command[0].try_into().unwrap();
            match command_type {
                Command::Input => {
                    let input = command[1];
                    let start_frame = command[2];
                    let length = command[3];
                    if !self.input_started
                        && (visual_frame >= start_frame)
                        && (visual_frame <= (start_frame + length))
                    {
                        self.input_started = true;
                        // write input to luma input redirection
                        // touch screen
                        if input & 0x10000000 != 0 {
                            unsafe { (0xAE0F3078 as *mut u32).write(input) };
                        // hid
                        } else {
                            unsafe { (0xAE0F3074 as *mut u32).write(input) };
                        }
                    } else if visual_frame > start_frame + length {
                        // clear input redirection
                        unsafe { (0xAE0F3078 as *mut u32).write(0x2000000u32) };
                        unsafe { (0xAE0F3074 as *mut u32).write(0xFFFu32) };
                        self.input_started = false;
                        self.last_command_ended = start_frame + length;
                        self.command_index += 1
                    }
                }
                Command::ImpreciseInput => {
                    let input = command[1];
                    let start_frame = command[2];
                    let length = command[3];
                    // we have just moved to this input, set up start and end times
                    if self.imprecise_input_storage[0] == 0 {
                        // get_time returns milliseconds, calculate when it needs to start based on the distance from previous input
                        let frames_since_last_command_ended =
                            (start_frame - self.last_command_ended) as f64;
                        let milliseconds_per_frame = 1_000.0 / 60.0;
                        // get_time can be slow, so calling this every update may not be the best way to go about this
                        self.imprecise_input_storage[0] = get_time()
                            + ((frames_since_last_command_ended * milliseconds_per_frame) as u64);
                        // time it needs to end is just the length converted to milliseconds
                        self.imprecise_input_storage[1] = self.imprecise_input_storage[0]
                            + (((length as f64) * milliseconds_per_frame) as u64);
                    } else {
                        let current_time = get_time();
                        // touch screen press needs to be ended
                        if current_time >= self.imprecise_input_storage[1] {
                            // clear input redirection
                            unsafe { (0xAE0F3078 as *mut u32).write(0x2000000u32) };
                            unsafe { (0xAE0F3074 as *mut u32).write(0xFFFu32) };
                            self.imprecise_input_storage[0] = 0;
                            self.imprecise_input_storage[1] = 0;
                            self.inprecise_input_started = false;
                            self.last_command_ended = start_frame + length;
                            self.command_index += 1;
                        // touch screen press needs to be started, this only needs to be run once
                        } else if !self.inprecise_input_started
                            && current_time >= self.imprecise_input_storage[0]
                        {
                            self.inprecise_input_started = true;
                            // write input to luma input redirection
                            // touch screen
                            if input & 0x1000000 != 0 {
                                unsafe { (0xAE0F3078 as *mut u32).write(input) };
                            // hid
                            } else {
                                unsafe { (0xAE0F3074 as *mut u32).write(input) };
                            }
                        }
                    }
                }
                Command::Read => {
                    let address = command[1];
                    let length = command[2] as usize;
                    let mut send_buffer = [0u8; 128];
                    for i in 0..length {
                        send_buffer[i] = game.read((address as usize) + i).unwrap();
                    }
                    let tcp_socket = SOCKET.lock();
                    let _ = tcp_socket.send(&send_buffer, 0);
                    self.command_index += 1;
                }
                Command::WaitUntil => {
                    let frame = command[1];
                    if visual_frame >= frame {
                        self.last_command_ended = visual_frame;
                        self.command_index += 1;
                    }
                }
                Command::End => {}
            }
        }
    }
}
