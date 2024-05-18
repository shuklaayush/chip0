use p3_derive::EnumDispatch;
use std::fmt::Debug;

mod cpu;
mod draw;
mod frame_buffer;
// mod hash;
mod keypad;
mod memory;
mod range;

use self::{
    cpu::CpuChip, draw::DrawChip, frame_buffer::FrameBufferChip, keypad::KeypadChip,
    memory::MemoryChip, range::RangeChip,
};

#[derive(Clone, Debug, EnumDispatch)]
pub enum Chip0MachineChip {
    Cpu(CpuChip),
    Draw(DrawChip),
    Keypad(KeypadChip),
    Range(RangeChip),
    Memory(MemoryChip),
    FrameBuffer(FrameBufferChip),
}
