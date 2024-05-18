use chip8_core::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH, FLAG_REGISTER, PROGRAM_START_ADDRESS},
    error::Chip8Error,
    input::InputKind,
    keypad::Key,
    state::{Address, SimpleState, State, Word},
};
use p3_field::PrimeField32;
use p3_matrix::dense::RowMajorMatrix;
use std::sync::{Arc, RwLock};

use crate::chips::{
    cpu::columns::{CpuCols, NUM_CPU_COLS},
    draw::columns::{DrawCols, NUM_DRAW_COLS},
    frame_buffer::columns::NUM_FRAME_BUFFER_COLS,
    keypad::columns::{KeypadCols, NUM_KEYPAD_COLS},
    memory::columns::NUM_MEMORY_COLS,
    range::columns::{RangeCols, NUM_RANGE_COLS},
};

#[derive(Default)]
pub struct IncrementalTrace<Cols: Default> {
    trace: Vec<Cols>,
    curr_row: Cols,
    next_row: Cols,
}

// TODO: Derive simple state from traces
pub struct StarkState<F: PrimeField32> {
    simple_state: SimpleState,

    cpu_trace: IncrementalTrace<CpuCols<F>>,
    draw_trace: IncrementalTrace<DrawCols<F>>,
    keypad_trace: IncrementalTrace<KeypadCols<F>>,
    // range_trace: IncrementalTrace<RangeCols<F>>,
    // memory_trace: RowMajorMatrix<F>,
    // frame_buffer_trace: RowMajorMatrix<F>,
}

impl<F: PrimeField32> Default for StarkState<F> {
    fn default() -> Self {
        let mut cpu_trace: IncrementalTrace<CpuCols<F>> = IncrementalTrace::default();
        cpu_trace.curr_row.program_counter = F::from_canonical_u16(PROGRAM_START_ADDRESS);

        Self {
            simple_state: SimpleState::default(),
            cpu_trace,
            draw_trace: IncrementalTrace::default(),
            keypad_trace: IncrementalTrace::default(),
            // range_trace: IncrementalTrace::default(),
        }
    }
}

impl<F: PrimeField32> State for StarkState<F> {
    fn load_rom(&mut self, bytes: &[u8]) -> Result<(), Chip8Error> {
        self.simple_state.load_rom(bytes)
    }

    fn clk(&self) -> Result<u64, Chip8Error> {
        self.simple_state.clk()
    }

    fn clk_ptr(&self) -> Arc<RwLock<u64>> {
        self.simple_state.clk_ptr()
    }

    fn sound_timer_ptr(&self) -> Arc<RwLock<Word>> {
        self.simple_state.sound_timer_ptr()
    }

    fn frame_buffer_ptr(&self) -> Arc<RwLock<[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>> {
        self.simple_state.frame_buffer_ptr()
    }

    fn program_counter(&self) -> Address {
        self.simple_state.program_counter()
    }

    fn delay_timer(&self) -> Word {
        self.simple_state.delay_timer()
    }

    fn sound_timer(&self) -> Result<Word, Chip8Error> {
        self.simple_state.sound_timer()
    }

    fn memory(&self, addr: Address) -> Result<Word, Chip8Error> {
        self.simple_state.memory(addr)
    }

    fn register(&self, index: Word) -> Word {
        self.simple_state.register(index)
    }

    fn index_register(&self) -> Address {
        self.simple_state.index_register()
    }

    fn key(&self, index: Word) -> bool {
        self.simple_state.key(index)
    }

    fn frame_buffer(&self, y: usize, x: usize) -> Result<bool, Chip8Error> {
        self.simple_state.frame_buffer(y, x)
    }

    fn set_frame_buffer(&self, y: usize, x: usize, bit: bool) -> Result<(), Chip8Error> {
        // ((*self.frame_buffer).checked_write()?)[y][x] = bit;

        self.simple_state.set_frame_buffer(y, x, bit)
    }

    fn set_program_counter(&mut self, pc: Address) {
        let next_row = &mut self.cpu_trace.next_row;
        next_row.program_counter = F::from_canonical_u16(pc);

        self.simple_state.set_program_counter(pc)
    }

    fn set_delay_timer(&mut self, value: Word) {
        let curr_row = &mut self.cpu_trace.curr_row;
        curr_row.delay_timer = F::from_canonical_u8(value);

        self.simple_state.set_delay_timer(value)
    }

    fn set_sound_timer(&mut self, value: Word) -> Result<(), Chip8Error> {
        let curr_row = &mut self.cpu_trace.curr_row;
        curr_row.sound_timer = F::from_canonical_u8(value);

        self.simple_state.set_sound_timer(value)
    }

    fn set_index_register(&mut self, addr: Address) {
        let curr_row = &mut self.cpu_trace.curr_row;
        curr_row.index_register = F::from_canonical_u16(addr);

        self.simple_state.set_index_register(addr)
    }

    fn set_register(&mut self, index: Word, value: Word) {
        let curr_row = &mut self.cpu_trace.curr_row;
        curr_row.registers[index as usize] = F::from_canonical_u8(value);

        self.simple_state.set_register(index, value)
    }

    fn set_flag_register(&mut self, flag: bool) {
        let curr_row = &mut self.cpu_trace.curr_row;
        curr_row.registers[FLAG_REGISTER] = F::from_bool(flag);

        self.simple_state.set_flag_register(flag)
    }

    fn set_memory(&mut self, addr: Address, value: Word) -> Result<(), Chip8Error> {
        // if (addr as usize) < MEMORY_SIZE {
        //     self.memory[addr as usize] = value;
        //     Ok(())
        // } else {
        //     Err(Chip8Error::MemoryAccessOutOfBounds(addr))
        // }

        self.simple_state.set_memory(addr, value)
    }

    fn set_key(&mut self, key: Key, kind: InputKind) {
        self.cpu_trace.curr_row.keypad[key as usize] = F::from_bool(kind == InputKind::Press);

        self.keypad_trace.curr_row.index = F::from_canonical_usize(key as usize);
        self.keypad_trace.curr_row.value = F::from_bool(kind == InputKind::Press);
        self.keypad_trace.add_curr_row_to_trace();

        self.simple_state.set_key(key, kind)
    }

    fn clear_framebuffer(&mut self) -> Result<(), Chip8Error> {
        // *self.frame_buffer.checked_write()? = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

        self.simple_state.clear_framebuffer()
    }

    fn push_stack(&mut self, addr: Address) {
        let curr_row = &mut self.cpu_trace.curr_row;
        let next_row = &mut self.cpu_trace.next_row;
        curr_row.stack_pointer += F::one();
        curr_row.stack[self.simple_state.stack_pointer as usize] = curr_row.program_counter;
        next_row.program_counter = F::from_canonical_u16(addr);

        self.simple_state.push_stack(addr)
    }

    fn pop_stack(&mut self) {
        let curr_row = &mut self.cpu_trace.curr_row;
        let next_row = &mut self.cpu_trace.next_row;
        curr_row.stack_pointer -= F::one();
        next_row.program_counter = F::from_canonical_u16(
            self.simple_state.stack[self.simple_state.stack_pointer as usize],
        );

        self.simple_state.pop_stack()
    }

    fn increment_program_counter(&mut self) {
        let curr_row = &self.cpu_trace.curr_row;
        let next_row = &mut self.cpu_trace.next_row;
        next_row.program_counter = curr_row.program_counter + F::one();

        self.simple_state.increment_program_counter()
    }

    fn increment_clk(&mut self) -> Result<(), Chip8Error> {
        let curr_row = &self.cpu_trace.curr_row;
        let next_row = &mut self.cpu_trace.next_row;
        next_row.clk = curr_row.clk + F::one();

        self.cpu_trace.add_curr_row_to_trace();

        self.simple_state.increment_clk()
    }

    fn decrement_delay_timer(&mut self) {
        let curr_row = &mut self.cpu_trace.curr_row;
        curr_row.delay_timer -= F::one();

        self.simple_state.decrement_delay_timer()
    }

    fn decrement_sound_timer(&mut self) -> Result<(), Chip8Error> {
        let curr_row = &mut self.cpu_trace.curr_row;
        curr_row.sound_timer -= F::one();

        self.simple_state.decrement_sound_timer()
    }
}

impl<F: PrimeField32> IncrementalTrace<CpuCols<F>> {
    pub fn add_curr_row_to_trace(&mut self) {
        self.trace.push(self.curr_row);
        self.curr_row = self.next_row;

        self.next_row = CpuCols::default();
        // TODO: Probably wrong
        // Copy state
        self.next_row.registers = self.curr_row.registers;
        self.next_row.index_register = self.curr_row.index_register;
        self.next_row.stack = self.curr_row.stack;
        self.next_row.stack_pointer = self.curr_row.stack_pointer;
        self.next_row.keypad = self.curr_row.keypad;
    }
}

impl<F: PrimeField32> IncrementalTrace<KeypadCols<F>> {
    pub fn add_curr_row_to_trace(&mut self) {
        self.trace.push(self.curr_row);
        self.curr_row = self.next_row;

        self.next_row = KeypadCols::default();
    }
}

impl<F: PrimeField32> IncrementalTrace<DrawCols<F>> {
    pub fn add_curr_row_to_trace(&mut self) {
        self.trace.push(self.curr_row);
        self.curr_row = self.next_row;

        self.next_row = DrawCols::default();
    }
}
