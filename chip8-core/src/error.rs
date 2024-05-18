use thiserror::Error;

use crate::state::Address;

#[derive(Error, Debug, Clone)]
pub enum Chip8Error {
    #[error("Memory access out of bounds: 0x{0:04X}")]
    MemoryAccessOutOfBounds(Address),
    #[error("Unimplemented opcode: 0x{0:04X}")]
    UnimplementedOpcode(u16),
    #[error("ROM size too big: {0}bytes")]
    RomTooBig(usize),
    #[error("Display Error: {0}")]
    DisplayError(String),
    #[error("Input Error: {0}")]
    InputError(String),
    #[error("Audio Error: {0}")]
    AudioError(String),
    #[error("Async/Await Error: {0}")]
    AsyncAwaitError(String),
    #[error("Mutex read error: {0}")]
    MutexReadError(String),
    #[error("Mutex write error: {0}")]
    MutexWriteError(String),
    #[error("Interrupted")]
    Interrupt,
}
