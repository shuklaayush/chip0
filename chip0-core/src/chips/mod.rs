use p3_derive::EnumDispatch;
use std::fmt::Debug;

pub mod cpu;
pub mod draw;
pub mod frame_buffer;
// pub mod hash;
pub mod keypad;
pub mod memory;
pub mod range;

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
