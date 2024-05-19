use p3_field::PrimeField32;
use p3_machine::machine::Machine;
use p3_uni_stark::{StarkGenericConfig, Val};

use crate::chips::{
    cpu::CpuChip, draw::DrawChip, frame_buffer::FrameBufferChip, keypad::KeypadChip,
    memory::MemoryChip, range::RangeChip, Chip0MachineChip,
};

#[derive(Default, Clone)]
pub struct Chip0Machine {}

pub enum Chip0MachineBus {
    DrawBus = 0,
    KeypadBus = 1,
    MemoryBus = 2,
    FrameBufferBus = 3,
    RangeBus = 4,
    // HashBus = 5,
}

impl<'a, SC> Machine<'a, SC, Chip0MachineChip> for Chip0Machine
where
    SC: StarkGenericConfig,
    Val<SC>: PrimeField32,
{
    fn chips(&self) -> Vec<Chip0MachineChip> {
        let cpu_chip = CpuChip {};
        let draw_chip = DrawChip {};
        let keypad_chip = KeypadChip {};
        let memory_chip = MemoryChip {
            bus_range: Chip0MachineBus::RangeBus as usize,
        };
        let frame_buffer_chip = FrameBufferChip {
            bus_range: Chip0MachineBus::RangeBus as usize,
        };
        let range_chip = RangeChip {
            bus_range: Chip0MachineBus::RangeBus as usize,
        };

        vec![
            Chip0MachineChip::Cpu(cpu_chip),
            Chip0MachineChip::Draw(draw_chip),
            Chip0MachineChip::Keypad(keypad_chip),
            Chip0MachineChip::Memory(memory_chip),
            Chip0MachineChip::FrameBuffer(frame_buffer_chip),
            Chip0MachineChip::Range(range_chip),
        ]
    }
}
