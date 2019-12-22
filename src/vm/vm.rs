use std::error::Error;
use crate::vm::rom::EMULATOR_ROM;

#[derive(Debug)]
struct VM {
    /// ROM memory for CHIP-8. This holds the reserved 512 bytes as
    /// well as the program memory. It is a pristine state upon being
    /// loaded that Memory can be reset back to.
    rom: [uint8; 0x1000],

    /// The ROM size.
    rom_size: int32,

    /// Memory addressable by CHIP-8. The first 512 bytes are reserved
    /// for the font sprites, any RCA 1802 code, and the stack.
    memory: [uint8; 0x1000],

    /// Video memory for CHIP-8 (64x32 bits). Each bit represents a
    /// single pixel. It is stored MSB first. For example, pixel <0,0>
    /// is bit 0x80 of byte 0. 4x the video memory is used for the
    /// CHIP-48, which is 128x64 resolution. There are 4 extra lines
    /// to prevent overflows when scrolling.
    video: [uint8; 0x440],

    /// The stack was in a reserved section of memory on the 1802.
    /// Originally it was only 12-cells deep, but later implementations
    /// went as high as 16-cells.
    stack: [uint; 16],

    /// The stack pointer.
    sp: uint,

    /// The program counter, which always begins at 0x200.
    pc: uint,

    /// The VM registers.
    regs: Registers,

    /// Clock is the time (in ns) when emulation begins.
    clock: int64,

    /// Cycles is how many clock cycles have been processed. It is assumed
    /// one clock cycle per instruction.
    cycles: int64,

    /// Speed is how many cycles (instructions) should execute per second.
    /// By default this is 700. The RCA CDP1802 ran at 1.76 MHz, with each
    /// instruction taking 16-24 clock cycles, which is a bit over 70,000
    /// instructions per second.
    speed: int64,

    /// Keys hold the current state for the 16-key pad keys.
    keys: [bool; 16],

    /// Number of bytes per scan line. This is 8 in low mode and 16 when high.
    pitch: int32
}

#[derive(Debug)]
struct Registers {
    /// I is the address register.
    i: uint,

    /// V are the 16 virtual registers.
    v: [uint8; 16],

    /// R are the 8, HP-RPL user flags.
    r: [uint8; 8],

    /// DT is the delay timer register. It is set to a time (in ns) in the
    /// future and compared against the current time.
    dt: int64,

    /// ST is the sound timer register. It is set to a time (in ns) in the
    /// future and compared against the current time.
    st: int64
}

pub fn load_rom(program: Vec<uint8>) -> Result<VM, Error> {
    // Check if the program fits within memory
    if program.len() > 0x800 {
        return Err("The program is too large to fit into memory")
    }

    let mut vm = VM::new();

    vm.rom_size = program.len();
    vm.rom[..0x200].clone_from_slice(&EMULATOR_ROM);
    vm.rom[0x200..].clone_from_slice(&program);

    Ok(vm)
}

impl VM {
    pub fn new() -> VM {
        VM {
            rom: [0; 0x1000],
            rom_size: 0,
            memory: [0; 0x1000],
            video: [0; 0x440],
            stack: [0; 16],
            sp: 0,
            pc: 0x200,
            regs: Registers::new(),
            clock: (),
            cycles: (),
            speed: 700,
            keys: [false; 16],
            pitch: 8
        }
    }
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            i: 0,
            v: [0; 16],
            r: [0; 8],
            dt: 0,
            st: 0
        }
    }
}