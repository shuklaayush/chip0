use p3_field::PrimeField32;
use p3_machine::machine::Machine;
use p3_uni_stark::{StarkGenericConfig, Val};

use crate::{
    bus::Chip0MachineBus,
    chips::{
        clear::ClearChip, cpu::CpuChip, draw::DrawChip, frame_buffer::FrameBufferChip,
        keypad::KeypadChip, memory::MemoryChip, memory_start::MemoryStartChip, range::RangeChip,
        Chip0MachineChip,
    },
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

impl<'a, SC> Machine<'a, SC> for Chip0Machine
where
    SC: StarkGenericConfig,
    Val<SC>: PrimeField32,
{
    type Chip = Chip0MachineChip;
    type Bus = Chip0MachineBus;

    fn chips(&self) -> Vec<Chip0MachineChip> {
        let cpu_chip = CpuChip::new(
            Chip0MachineBus::ClearBus as usize,
            Chip0MachineBus::DrawBus as usize,
            Chip0MachineBus::MemoryBus as usize,
            Chip0MachineBus::KeypadBus as usize,
        );
        let clear_chip = ClearChip::new(
            Chip0MachineBus::ClearBus as usize,
            Chip0MachineBus::FrameBufferBus as usize,
        );
        let draw_chip = DrawChip::new(
            Chip0MachineBus::DrawBus as usize,
            Chip0MachineBus::FrameBufferBus as usize,
            Chip0MachineBus::MemoryBus as usize,
        );
        let keypad_chip = KeypadChip::new(Chip0MachineBus::KeypadBus as usize);
        let memory_chip = MemoryChip::new(
            Chip0MachineBus::MemoryStartBus as usize,
            Chip0MachineBus::MemoryBus as usize,
            Chip0MachineBus::RangeBus as usize,
        );
        let frame_buffer_chip = FrameBufferChip::new(
            Chip0MachineBus::FrameBufferBus as usize,
            Chip0MachineBus::RangeBus as usize,
        );
        let range_chip = RangeChip::new(Chip0MachineBus::RangeBus as usize);
        let memory_start_chip =
            MemoryStartChip::new(self.rom.clone(), Chip0MachineBus::MemoryStartBus as usize);

        vec![
            Chip0MachineChip::Cpu(cpu_chip),
            Chip0MachineChip::Clear(clear_chip),
            Chip0MachineChip::Draw(draw_chip),
            Chip0MachineChip::Keypad(keypad_chip),
            Chip0MachineChip::Memory(memory_chip),
            Chip0MachineChip::FrameBuffer(frame_buffer_chip),
            Chip0MachineChip::Range(range_chip),
            Chip0MachineChip::MemoryStart(memory_start_chip),
        ]
    }
}
