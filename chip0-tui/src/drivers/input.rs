use chip8_core::{
    drivers::InputDriver,
    error::Chip8Error,
    input::{InputEvent, InputKind},
    keypad::Key,
};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::io::Write;

const FREQUENCY: u64 = 120;

fn keymap(c: char) -> Option<Key> {
    match c {
        '1' => Some(Key::Key1),
        '2' => Some(Key::Key2),
        '3' => Some(Key::Key3),
        '4' => Some(Key::KeyC),
        'Q' => Some(Key::Key4),
        'W' => Some(Key::Key5),
        'E' => Some(Key::Key6),
        'R' => Some(Key::KeyD),
        'A' => Some(Key::Key7),
        'S' => Some(Key::Key8),
        'D' => Some(Key::Key9),
        'F' => Some(Key::KeyA),
        'Z' => Some(Key::KeyA),
        'X' => Some(Key::Key0),
        'C' => Some(Key::KeyB),
        'V' => Some(Key::KeyF),
        _ => None,
    }
}

#[derive(Serialize, Deserialize)]
pub struct CsvRecord {
    pub clk: u64,
    pub key: char,
    pub kind: u8,
}

#[derive(Default)]
pub struct TerminalKeyboardInput<W: Write> {
    writer: Option<Writer<W>>,
}

impl<W: Write> TerminalKeyboardInput<W> {
    pub fn new(writer: Option<Writer<W>>) -> Self {
        Self { writer }
    }
}

impl<W: Write + Send> InputDriver for TerminalKeyboardInput<W> {
    fn frequency(&self) -> u64 {
        FREQUENCY
    }

    fn log_input(&mut self, clk: u64, input: InputEvent) -> Result<(), Chip8Error> {
        if let Some(writer) = &mut self.writer {
            let record = CsvRecord {
                clk,
                key: char::from(input.key),
                kind: input.kind as u8,
            };
            writer
                .serialize(record)
                .map_err(|e| Chip8Error::InputError(e.to_string()))
        } else {
            Ok(())
        }
    }

    fn poll(&mut self) -> Result<Option<InputEvent>, Chip8Error> {
        let event = read().map_err(|e| Chip8Error::InputError(e.to_string()))?;
        if let Event::Key(KeyEvent {
            code,
            kind,
            modifiers,
            ..
        }) = event
        {
            match (modifiers, code) {
                (KeyModifiers::CONTROL, KeyCode::Char('c')) => return Err(Chip8Error::Interrupt),
                (_, KeyCode::Esc) => return Err(Chip8Error::Interrupt),
                (_, KeyCode::Char(c)) => {
                    let kind = match kind {
                        KeyEventKind::Press => Some(InputKind::Press),
                        KeyEventKind::Release => Some(InputKind::Release),
                        _ => None,
                    };

                    if let Some(kind) = kind {
                        if let Some(key) = keymap(c.to_ascii_uppercase()) {
                            let event = InputEvent { key, kind };
                            return Ok(Some(event));
                        }
                    }
                }
                _ => return Ok(None),
            }
        }

        Ok(None)
    }
}
