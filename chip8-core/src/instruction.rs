use crate::state::{Address, Word};

type Nibble = u8; // ideally u4
type RegisterIndex = u8; // ideally u4

#[derive(Debug)]
pub enum Instruction {
    ClearDisplay,
    Return,
    Jump(Address),
    Call(Address),
    SkipEqual(RegisterIndex, Word),
    SkipNotEqual(RegisterIndex, Word),
    SkipEqualXY(RegisterIndex, RegisterIndex),
    Load(RegisterIndex, Word),
    Add(RegisterIndex, Word),

    Move(RegisterIndex, RegisterIndex),
    Or(RegisterIndex, RegisterIndex),
    And(RegisterIndex, RegisterIndex),
    Xor(RegisterIndex, RegisterIndex),
    AddXY(RegisterIndex, RegisterIndex),
    SubXY(RegisterIndex, RegisterIndex),
    ShiftRight(RegisterIndex),
    SubYX(RegisterIndex, RegisterIndex),
    ShiftLeft(RegisterIndex),

    SkipNotEqualXY(RegisterIndex, RegisterIndex),
    LoadI(Address),
    JumpV0(Address),
    Random(RegisterIndex, Word),
    Draw(RegisterIndex, RegisterIndex, Nibble),

    SkipKeyPressed(RegisterIndex),
    SkipKeyNotPressed(RegisterIndex),

    LoadDelay(RegisterIndex),
    WaitKeyPress(RegisterIndex),
    SetDelay(RegisterIndex),
    SetSound(RegisterIndex),
    AddI(RegisterIndex),
    LoadFont(RegisterIndex),
    StoreBCD(RegisterIndex),
    StoreRegisters(RegisterIndex),
    LoadMemory(RegisterIndex),
}
