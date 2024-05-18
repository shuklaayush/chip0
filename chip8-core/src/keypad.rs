use std::fmt::Display;

use crate::error::Chip8Error;

// ╔═══╦═══╦═══╦═══╗
// ║ 1 ║ 2 ║ 3 ║ C ║
// ╠═══╬═══╬═══╬═══╣
// ║ 4 ║ 5 ║ 6 ║ D ║
// ╠═══╬═══╬═══╬═══╣
// ║ 7 ║ 8 ║ 9 ║ E ║
// ╠═══╬═══╬═══╬═══╣
// ║ A ║ 0 ║ B ║ F ║
// ╚═══╩═══╩═══╩═══╝
#[derive(Debug, Clone, Copy)]
pub enum Key {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
}

impl From<Key> for char {
    fn from(key: Key) -> Self {
        match key {
            Key::Key0 => '0',
            Key::Key1 => '1',
            Key::Key2 => '2',
            Key::Key3 => '3',
            Key::Key4 => '4',
            Key::Key5 => '5',
            Key::Key6 => '6',
            Key::Key7 => '7',
            Key::Key8 => '8',
            Key::Key9 => '9',
            Key::KeyA => 'A',
            Key::KeyB => 'B',
            Key::KeyC => 'C',
            Key::KeyD => 'D',
            Key::KeyE => 'E',
            Key::KeyF => 'F',
        }
    }
}

impl TryFrom<char> for Key {
    type Error = Chip8Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Key0),
            '1' => Ok(Self::Key1),
            '2' => Ok(Self::Key2),
            '3' => Ok(Self::Key3),
            '4' => Ok(Self::Key4),
            '5' => Ok(Self::Key5),
            '6' => Ok(Self::Key6),
            '7' => Ok(Self::Key7),
            '8' => Ok(Self::Key8),
            '9' => Ok(Self::Key9),
            'A' => Ok(Self::KeyA),
            'B' => Ok(Self::KeyB),
            'C' => Ok(Self::KeyC),
            'D' => Ok(Self::KeyD),
            'E' => Ok(Self::KeyE),
            'F' => Ok(Self::KeyF),
            _ => Err(Chip8Error::InputError("Unrecognized key".to_string())),
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = char::from(*self);
        write!(f, "{key}")
    }
}
