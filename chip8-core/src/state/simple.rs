use std::sync::{Arc, RwLock};

use super::{Address, State, Word};
use crate::{
    constants::{
        DISPLAY_HEIGHT, DISPLAY_WIDTH, FLAG_REGISTER, FONTSET, FONTSET_START_ADDRESS, MEMORY_SIZE,
        NUM_KEYS, NUM_REGISTERS, OPCODE_SIZE, PROGRAM_START_ADDRESS, STACK_DEPTH,
    },
    error::Chip8Error,
    input::InputKind,
    keypad::Key,
    rwlock::{CheckedRead, CheckedWrite},
};

// TODO: Compare performance with atomics, channels instead of locks
pub struct SimpleState {
    /// Cycle counter to keep track of the number of CPU cycles executed.
    // TODO: Make private
    pub clk: Arc<RwLock<u64>>,
    pub registers: [Word; NUM_REGISTERS],
    pub memory: [Word; MEMORY_SIZE],
    pub index_register: Address,
    pub program_counter: Address,
    pub stack: [Address; STACK_DEPTH],
    pub stack_pointer: Word,
    pub delay_timer: Word,
    pub sound_timer: Arc<RwLock<Word>>,
    pub keypad: [bool; NUM_KEYS],
    pub frame_buffer: Arc<RwLock<[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>>,
}

impl Default for SimpleState {
    fn default() -> Self {
        let mut memory = [0; MEMORY_SIZE];
        let start = FONTSET_START_ADDRESS as usize;
        let end = FONTSET_START_ADDRESS as usize + FONTSET.len();
        memory[start..end].copy_from_slice(FONTSET.as_slice());

        Self {
            clk: Arc::new(RwLock::new(0)),
            registers: [0; NUM_REGISTERS],
            memory,
            index_register: 0,
            program_counter: PROGRAM_START_ADDRESS,
            stack: [0; STACK_DEPTH],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: Arc::new(RwLock::new(0)),
            keypad: [false; NUM_KEYS],
            frame_buffer: Arc::new(RwLock::new([[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT])),
        }
    }
}

impl State for SimpleState {
    fn load_rom(&mut self, bytes: &[u8]) -> Result<(), Chip8Error> {
        let start = PROGRAM_START_ADDRESS as usize;
        let end = PROGRAM_START_ADDRESS as usize + bytes.len();

        if end > MEMORY_SIZE {
            Err(Chip8Error::RomTooBig(bytes.len()))
        } else {
            self.memory[start..end].copy_from_slice(bytes);
            Ok(())
        }
    }

    fn clk(&self) -> Result<u64, Chip8Error> {
        let clk = *self.clk.checked_read()?;
        Ok(clk)
    }

    fn clk_ptr(&self) -> Arc<RwLock<u64>> {
        self.clk.clone()
    }

    fn sound_timer_ptr(&self) -> Arc<RwLock<Word>> {
        self.sound_timer.clone()
    }

    fn frame_buffer_ptr(&self) -> Arc<RwLock<[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>> {
        self.frame_buffer.clone()
    }

    fn program_counter(&self) -> Address {
        self.program_counter
    }

    fn delay_timer(&self) -> Word {
        self.delay_timer
    }

    fn sound_timer(&self) -> Result<Word, Chip8Error> {
        let st = *self.sound_timer.checked_read()?;
        Ok(st)
    }

    fn memory(&mut self, addr: Address) -> Result<Word, Chip8Error> {
        if (addr as usize) < MEMORY_SIZE {
            Ok(self.memory[addr as usize])
        } else {
            Err(Chip8Error::MemoryAccessOutOfBounds(addr))
        }
    }

    fn register(&self, index: Word) -> Word {
        self.registers[index as usize]
    }

    fn index_register(&self) -> Address {
        self.index_register
    }

    fn key(&self, index: Word) -> bool {
        self.keypad[index as usize]
    }

    fn frame_buffer(&mut self, y: usize, x: usize) -> Result<bool, Chip8Error> {
        let fb = ((*self.frame_buffer).checked_read()?)[y][x];
        Ok(fb)
    }

    fn set_frame_buffer(&mut self, y: usize, x: usize, bit: bool) -> Result<(), Chip8Error> {
        ((*self.frame_buffer).checked_write()?)[y][x] = bit;
        Ok(())
    }

    fn set_program_counter(&mut self, pc: Address) {
        self.program_counter = pc;
    }

    fn set_delay_timer(&mut self, value: Word) {
        self.delay_timer = value;
    }

    fn set_sound_timer(&mut self, value: Word) -> Result<(), Chip8Error> {
        *self.sound_timer.checked_write()? = value;
        Ok(())
    }

    fn set_index_register(&mut self, addr: Address) {
        self.index_register = addr;
    }

    fn set_register(&mut self, index: Word, value: Word) {
        self.registers[index as usize] = value;
    }

    fn set_flag_register(&mut self, flag: bool) {
        self.registers[FLAG_REGISTER] = flag as Word;
    }

    fn set_memory(&mut self, addr: Address, value: Word) -> Result<(), Chip8Error> {
        if (addr as usize) < MEMORY_SIZE {
            self.memory[addr as usize] = value;
            Ok(())
        } else {
            Err(Chip8Error::MemoryAccessOutOfBounds(addr))
        }
    }

    fn set_key(&mut self, key: Key, kind: InputKind) {
        self.keypad[key as usize] = kind == InputKind::Press;
    }

    fn clear_framebuffer(&mut self) -> Result<(), Chip8Error> {
        *self.frame_buffer.checked_write()? = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
        Ok(())
    }

    fn push_stack(&mut self, addr: Address) {
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = addr;
    }

    fn pop_stack(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];
    }

    fn increment_program_counter(&mut self) {
        self.program_counter += OPCODE_SIZE;
    }

    fn increment_clk(&mut self) -> Result<(), Chip8Error> {
        *self.clk.checked_write()? += 1;
        Ok(())
    }

    fn decrement_delay_timer(&mut self) {
        self.delay_timer -= 1;
    }

    fn decrement_sound_timer(&mut self) -> Result<(), Chip8Error> {
        *self.sound_timer.checked_write()? -= 1;
        Ok(())
    }
}
