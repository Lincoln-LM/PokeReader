from math import floor
import socket
import struct
from enum import Enum
from time import time
from typing import Union

COMMAND_BUFFER_SIZE = 1024
READ_BUFFER_SIZE = 128

class Button(Enum):
    A = 1,
    B = 2,
    Select = 4,
    Start = 8,
    Dright = 16,
    Dleft = 32,
    Dup = 64,
    Ddown = 128,
    R = 256,
    L = 512,
    X = 1024,
    Y = 2048,

class Command(Enum):
    # Blank command symboling end of command list
    End = 0,
    # Send hid/touch input at specific frame for specific length
    Input = 1,
    # Send hid/touch input based on os::get_time()
    ImpreciseInput = 2,
    # Read value from memory
    Read = 3,

    # Wait until specific frame before moving to next command
    WaitUntil = 255,

class PKRDClient:
    def __init__(self, ip, port = 7000, connect = True):
        self.ip = ip
        self.port = port
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.command_buffer = b''
        self.reads = {}
        self.command_count = 0
        if connect:
            self.connect()

    def connect(self):
        self.socket.connect((self.ip, self.port))

    def close(self):
        self.socket.close()

    def add_command(self, command_type: Command, command_params):
        if len(self.command_buffer) >= COMMAND_BUFFER_SIZE:
            raise Exception("Command limit reached")
        self.command_count += 1
        if command_type in (Command.Input, Command.ImpreciseInput):
            button: Union[Button, int]
            start_frame: int
            length: int
            button, start_frame, length = command_params
            if isinstance(button, Button):
                button_value: int = button.value[0] ^ 0xFFF
            else:
                button_value: int = button
            self.command_buffer += struct.pack("LLLL", command_type.value[0], button_value, start_frame, length)
        elif command_type == Command.Read:
            address: int
            length: int
            name: str
            address, length, name = command_params
            self.command_buffer += struct.pack("LLLL", command_type.value[0], address, length, 0)
            self.reads[name] = length
        elif command_type == Command.WaitUntil:
            frame: int
            frame, = command_params
            self.command_buffer += struct.pack("LLLL", command_type.value[0], frame, 0, 0)
        else:
            raise NotImplementedError()
    
    def send_commands(self):
        self.socket.sendall(self.command_buffer)
        command_count = struct.unpack("I", self.socket.recv(4))[0]
        assert command_count == self.command_count
        self.command_buffer = b''
        self.command_count = 0

    def read_next_value(self):
        name = list(self.reads.keys())[0]
        result = (name, self.socket.recv(READ_BUFFER_SIZE)[:self.reads.pop(name)])
        return result

    def read_all_values(self):
        values = {}
        read_keys = list(self.reads.keys())
        for name in read_keys:
            _name, value = self.read_next_value()
            assert _name == name
            values[name] = value
        return values

    @staticmethod
    def touch(x, y):
        x = (x * 4096) // 320
        y = (y * 4096) // 240
        return x | (y << 12) | (1 << 24)

print("Connecting to pkrd")
client = PKRDClient("192.168.0.100")
print("Connected")

client.add_command(Command.Input, [Button.A, 1, 349])
client.add_command(Command.Input, [Button.A, 375, 8])
client.add_command(Command.Input, [Button.A, 450, 8])
client.add_command(Command.Input, [Button.A, 520, 8])
client.add_command(Command.Input, [Button.A, 660, 8])
client.add_command(Command.Input, [Button.A, 820, 8])
client.add_command(Command.Input, [Button.A, 900, 8])
client.add_command(Command.Input, [Button.A, 1050, 8])
client.add_command(Command.Input, [Button.A, 1175, 8])
client.add_command(Command.Input, [Button.A, 1600, 1])
client.add_command(Command.Read, [0xA23FAC + 0xDCF4, 2, 'dvs'])
client.add_command(Command.ImpreciseInput, [client.touch(160,160), 1660, 10])
client.add_command(Command.ImpreciseInput, [client.touch(160,160), 1720, 10])
client.add_command(Command.ImpreciseInput, [client.touch(190,190), 1880, 10])
client.send_commands()
print("PKRD recieved commands")
read_values = client.read_all_values()
print("All memory values read")
atkdef, spespc = struct.unpack("BB", read_values['dvs'])
print(f"DVs: {atkdef >> 4} {atkdef & 0xF} {spespc >> 4} {spespc & 0xF}")
