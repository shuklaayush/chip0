use p3_field::PrimeField32;
use p3_machine::machine::Machine;
use p3_uni_stark::{StarkGenericConfig, Val};

use crate::chips::{
    cpu::CpuChip, draw::DrawChip, frame_buffer::FrameBufferChip, keypad::KeypadChip,
    memory::MemoryChip, memory_start::MemoryStartChip, range::RangeChip, Chip0MachineChip,
};

#[derive(Default, Clone)]
pub struct Chip0Machine {
    pub rom: Vec<u8>,
}

impl Chip0Machine {
    pub fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }
}

pub enum Chip0MachineBus {
    DrawBus = 0,
    KeypadBus = 1,
    MemoryBus = 2,
    FrameBufferBus = 3,
    RangeBus = 4,
    MemoryStartBus = 5,
    // HashBus = 6,
}

impl<'a, SC> Machine<'a, SC, Chip0MachineChip> for Chip0Machine
where
    SC: StarkGenericConfig,
    Val<SC>: PrimeField32,
{
    fn chips(&self) -> Vec<Chip0MachineChip> {
        let cpu_chip = CpuChip {
            bus_draw: Chip0MachineBus::DrawBus as usize,
            bus_memory: Chip0MachineBus::MemoryBus as usize,
            bus_keypad: Chip0MachineBus::KeypadBus as usize,
        };
        let draw_chip = DrawChip {
            bus_draw: Chip0MachineBus::DrawBus as usize,
            bus_frame_buffer: Chip0MachineBus::FrameBufferBus as usize,
            bus_memory: Chip0MachineBus::MemoryBus as usize,
        };
        let keypad_chip = KeypadChip {
            bus_keypad: Chip0MachineBus::KeypadBus as usize,
        };
        let memory_chip = MemoryChip {
            bus_memory_start: Chip0MachineBus::MemoryStartBus as usize,
            bus_memory: Chip0MachineBus::MemoryBus as usize,
            bus_range: Chip0MachineBus::RangeBus as usize,
        };
        let frame_buffer_chip = FrameBufferChip {
            bus_frame_buffer: Chip0MachineBus::FrameBufferBus as usize,
            bus_range: Chip0MachineBus::RangeBus as usize,
        };
        let range_chip = RangeChip {
            bus_range: Chip0MachineBus::RangeBus as usize,
        };
        let memory_start_chip = MemoryStartChip {
            rom: self.rom.clone(),
            bus_memory_start: Chip0MachineBus::MemoryStartBus as usize,
        };

        vec![
            Chip0MachineChip::Cpu(cpu_chip),
            Chip0MachineChip::Draw(draw_chip),
            Chip0MachineChip::Keypad(keypad_chip),
            Chip0MachineChip::Memory(memory_chip),
            Chip0MachineChip::FrameBuffer(frame_buffer_chip),
            Chip0MachineChip::Range(range_chip),
            Chip0MachineChip::MemoryStart(memory_start_chip),
        ]
    }
}
