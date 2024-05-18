use std::sync::{Arc, RwLock};

use crate::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    error::Chip8Error,
    input::InputKind,
    keypad::Key,
};

mod simple;
pub use simple::SimpleState;

pub type Address = u16;
pub type Word = u8;

pub trait State: Default {
    fn load_rom(&mut self, bytes: &[u8]) -> Result<(), Chip8Error>;

    fn clk(&self) -> Result<u64, Chip8Error>;
    fn program_counter(&self) -> Address;
    fn delay_timer(&self) -> Word;
    fn sound_timer(&self) -> Result<Word, Chip8Error>;
    fn memory(&mut self, addr: Address) -> Result<Word, Chip8Error>;
    fn register(&self, index: Word) -> Word;
    fn index_register(&self) -> Address;
    fn key(&self, index: Word) -> bool;
    fn frame_buffer(&mut self, y: usize, x: usize) -> Result<bool, Chip8Error>;

    fn set_frame_buffer(&mut self, y: usize, x: usize, bit: bool) -> Result<(), Chip8Error>;
    fn set_program_counter(&mut self, pc: Address);
    fn set_delay_timer(&mut self, value: Word);
    fn set_sound_timer(&mut self, value: Word) -> Result<(), Chip8Error>;
    fn set_index_register(&mut self, addr: Address);
    fn set_register(&mut self, index: Word, value: Word);
    fn set_flag_register(&mut self, flag: bool);
    fn set_memory(&mut self, addr: Address, value: Word) -> Result<(), Chip8Error>;
    fn set_key(&mut self, key: Key, kind: InputKind);

    fn clear_framebuffer(&mut self) -> Result<(), Chip8Error>;
    fn push_stack(&mut self, addr: Address);
    fn pop_stack(&mut self);
    fn increment_program_counter(&mut self);
    fn increment_clk(&mut self) -> Result<(), Chip8Error>;
    fn decrement_delay_timer(&mut self);
    fn decrement_sound_timer(&mut self) -> Result<(), Chip8Error>;

    fn clk_ptr(&self) -> Arc<RwLock<u64>>;
    fn sound_timer_ptr(&self) -> Arc<RwLock<Word>>;
    fn frame_buffer_ptr(&self) -> Arc<RwLock<[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>>;
}
