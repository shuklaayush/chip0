pub const NUM_REGISTERS: usize = 16;
pub const MEMORY_SIZE: usize = 4096;
pub const STACK_DEPTH: usize = 16;
pub const OPCODE_SIZE: u16 = 2;

pub const FLAG_REGISTER: usize = 0xF;

pub const FONTSET_START_ADDRESS: u16 = 0x0;
pub const PROGRAM_START_ADDRESS: u16 = 0x200;

pub const NUM_KEYS: usize = 16;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub const FONT_SIZE: usize = 5;
const NUM_FONTS: usize = 16;
pub const FONTSET: [u8; NUM_FONTS * FONT_SIZE] = [
    0b11110000, // ████
    0b10010000, // █  █
    0b10010000, // █  █
    0b10010000, // █  █
    0b11110000, // ████
    //
    0b00100000, //   █
    0b01100000, //  ██
    0b00100000, //   █
    0b00100000, //   █
    0b01110000, //  ███
    //
    0b11110000, // ████
    0b00010000, //    █
    0b11110000, // ████
    0b10000000, // █
    0b11110000, // ████
    //
    0b11110000, // ████
    0b00010000, //    █
    0b11110000, // ████
    0b00010000, //    █
    0b11110000, // ████
    //
    0b10010000, // █  █
    0b10010000, // █  █
    0b11110000, // ████
    0b00010000, //    █
    0b00010000, //    █
    //
    0b11110000, // ████
    0b10000000, // █
    0b11110000, // ████
    0b00010000, //    █
    0b11110000, // ████
    //
    0b11110000, // ████
    0b10000000, // █
    0b11110000, // ████
    0b10010000, // █  █
    0b11110000, // ████
    //
    0b11110000, // ████
    0b00010000, //    █
    0b00100000, //   █
    0b01000000, //  █
    0b01000000, //  █
    //
    0b11110000, // ████
    0b10010000, // █  █
    0b11110000, // ████
    0b10010000, // █  █
    0b11110000, // ████
    //
    0b11110000, // ████
    0b10010000, // █  █
    0b11110000, // ████
    0b00010000, //    █
    0b11110000, // ████
    //
    0b11110000, // ████
    0b10010000, // █  █
    0b11110000, // ████
    0b10010000, // █  █
    0b10010000, // █  █
    //
    0b11100000, // ███
    0b10010000, // █  █
    0b11100000, // ███
    0b10010000, // █  █
    0b11100000, // ███
    //
    0b11110000, // ████
    0b10000000, // █
    0b10000000, // █
    0b10000000, // █
    0b11110000, // ████
    //
    0b11100000, // ███
    0b10010000, // █  █
    0b10010000, // █  █
    0b10010000, // █  █
    0b11100000, // ███
    //
    0b11110000, // ████
    0b10000000, // █
    0b11110000, // ████
    0b10000000, // █
    0b11110000, // ████
    //
    0b11110000, // ████
    0b10000000, // █
    0b11110000, // ████
    0b10000000, // █
    0b10000000, // █
];

pub const TICKS_PER_TIMER: u64 = 8;