use std::collections::VecDeque;

use crate::{error::Chip8Error, keypad::Key};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputKind {
    Release,
    Press,
}

impl TryFrom<u8> for InputKind {
    type Error = Chip8Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Release),
            1 => Ok(Self::Press),
            _ => Err(Chip8Error::InputError("Unsupported input kind".to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InputEvent {
    pub key: Key,
    pub kind: InputKind,
}

pub trait InputQueue {
    fn back_clk(&self) -> Option<u64>;
    fn enqueue(&mut self, clk: u64, event: InputEvent);
    fn dequeue(&mut self, current_clk: u64) -> Option<InputEvent>;
}

impl InputQueue for VecDeque<(u64, InputEvent)> {
    fn back_clk(&self) -> Option<u64> {
        self.back().map(|(clk, _)| *clk)
    }

    fn enqueue(&mut self, clk: u64, event: InputEvent) {
        self.push_back((clk, event));
    }

    fn dequeue(&mut self, current_clk: u64) -> Option<InputEvent> {
        if let Some((clk, _)) = self.front() {
            if *clk <= current_clk {
                let (_, event) = self.pop_front().unwrap();
                Some(event)
            } else {
                None
            }
        } else {
            None
        }
    }
}
