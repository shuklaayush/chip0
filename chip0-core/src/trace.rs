use chip8_core::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    error::Chip8Error,
    input::InputKind,
    keypad::Key,
    state::{Address, SimpleState, State, Word},
};
use p3_field::PrimeField32;
use p3_matrix::dense::RowMajorMatrix;
use std::sync::{Arc, RwLock};

pub struct StarkState<F: PrimeField32> {
    simple_state: SimpleState,

    cpu_trace: RowMajorMatrix<F>,
    draw_trace: RowMajorMatrix<F>,
    keypad_trace: RowMajorMatrix<F>,
    range_trace: RowMajorMatrix<F>,
    // memory_trace: RowMajorMatrix<F>,
    // frame_buffer_trace: RowMajorMatrix<F>,
}

impl<F: PrimeField32> Default for StarkState<F> {
    fn default() -> Self {
        Self {
            simple_state: SimpleState::default(),
            cpu_trace: RowMajorMatrix::new(vec![], 0),
            draw_trace: RowMajorMatrix::new(vec![], 0),
            keypad_trace: RowMajorMatrix::new(vec![], 0),
            range_trace: RowMajorMatrix::new(vec![], 0),
            // memory_trace: RowMajorMatrix::new(vec![], 0),
            // frame_buffer_trace: RowMajorMatrix::new(vec![], 0),
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
        self.simple_state.set_frame_buffer(y, x, bit)
    }

    fn set_program_counter(&mut self, pc: Address) {
        self.simple_state.set_program_counter(pc)
    }

    fn set_delay_timer(&mut self, value: Word) {
        self.simple_state.set_delay_timer(value)
    }

    fn set_sound_timer(&mut self, value: Word) -> Result<(), Chip8Error> {
        self.simple_state.set_sound_timer(value)
    }

    fn set_index_register(&mut self, addr: Address) {
        self.simple_state.set_index_register(addr)
    }

    fn set_register(&mut self, index: Word, value: Word) {
        self.simple_state.set_register(index, value)
    }

    fn set_flag_register(&mut self, flag: bool) {
        self.simple_state.set_flag_register(flag)
    }

    fn set_memory(&mut self, addr: Address, value: Word) -> Result<(), Chip8Error> {
        self.simple_state.set_memory(addr, value)
    }

    fn set_key(&mut self, key: Key, kind: InputKind) {
        self.simple_state.set_key(key, kind)
    }

    fn clear_framebuffer(&mut self) -> Result<(), Chip8Error> {
        self.simple_state.clear_framebuffer()
    }

    fn push_stack(&mut self, addr: Address) {
        self.simple_state.push_stack(addr)
    }

    fn pop_stack(&mut self) {
        self.simple_state.pop_stack()
    }

    fn increment_program_counter(&mut self) {
        self.simple_state.increment_program_counter()
    }

    fn increment_clk(&mut self) -> Result<(), Chip8Error> {
        self.simple_state.increment_clk()
    }

    fn decrement_delay_timer(&mut self) {
        self.simple_state.decrement_delay_timer()
    }

    fn decrement_sound_timer(&mut self) -> Result<(), Chip8Error> {
        self.simple_state.decrement_sound_timer()
    }
}
