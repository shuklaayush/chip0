use p3_derive::EnumDispatch;
use std::fmt::Debug;

pub mod clear;
pub mod cpu;
pub mod draw;
pub mod frame_buffer;
// pub mod hash;
pub mod keypad;
pub mod memory;
pub mod memory_start;
pub mod range;

use self::{
    clear::ClearChip, cpu::CpuChip, draw::DrawChip, frame_buffer::FrameBufferChip,
    keypad::KeypadChip, memory::MemoryChip, memory_start::MemoryStartChip, range::RangeChip,
};

#[derive(Clone, Debug, EnumDispatch)]
pub enum Chip0MachineChip {
    Cpu(CpuChip),
    Clear(ClearChip),
    Draw(DrawChip),
    Keypad(KeypadChip),
    Memory(MemoryChip),
    FrameBuffer(FrameBufferChip),
    Range(RangeChip),
    MemoryStart(MemoryStartChip),
}
