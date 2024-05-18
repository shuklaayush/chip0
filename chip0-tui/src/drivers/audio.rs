use chip8_core::{drivers::AudioDriver, error::Chip8Error};
use std::io::{stdout, Write};

const FREQUENCY: u64 = 60;

#[derive(Default)]
pub struct TerminalAudio {}

impl AudioDriver for TerminalAudio {
    fn frequency(&self) -> u64 {
        FREQUENCY
    }

    fn beep(&mut self) -> Result<(), Chip8Error> {
        let mut stdout = stdout();
        write!(stdout, "\x07").map_err(|e| Chip8Error::AudioError(e.to_string()))?;
        stdout
            .flush()
            .map_err(|e| Chip8Error::AudioError(e.to_string()))
    }
}
