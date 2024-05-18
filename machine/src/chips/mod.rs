use p3_derive::EnumDispatch;
use std::fmt::Debug;

mod cpu;

use cpu::CpuChip;

#[derive(Clone, Debug, EnumDispatch)]
pub enum Chip0MachineChip {
    Cpu(CpuChip),
}
