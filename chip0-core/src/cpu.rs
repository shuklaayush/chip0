use chip8_core::{
    constants::NUM_REGISTERS,
    cpu::Cpu,
    error::Chip8Error,
    instruction::Instruction,
    state::{State, Word},
};
use p3_field::PrimeField32;
use p3_matrix::dense::RowMajorMatrix;
use rand::Rng;
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use crate::trace::StarkState;

pub struct ProofRequest<F: PrimeField32> {
    pub traces: Vec<RowMajorMatrix<F>>,
    pub public_values: Vec<F>,
}

pub struct StarkCpu<R: Rng, F: PrimeField32> {
    state: StarkState<F>,
    clk_freq: u64,
    rng: R,
    proving_queue: Arc<RwLock<VecDeque<ProofRequest<F>>>>,
}

impl<R: Rng, F: PrimeField32> StarkCpu<R, F> {
    pub fn new(clk_freq: u64, rng: R) -> Self {
        Self {
            state: StarkState::default(),
            clk_freq,
            rng,
            proving_queue: Arc::new(RwLock::new(VecDeque::new())),
        }
    }
}

impl<R: Rng, F: PrimeField32> Cpu for StarkCpu<R, F> {
    type State = StarkState<F>;

    fn state(&mut self) -> &mut Self::State {
        &mut self.state
    }

    fn random(&mut self) -> Word {
        self.rng.gen()
    }

    fn frequency(&self) -> u64 {
        self.clk_freq
    }

    fn fetch(&mut self) -> Result<u16, Chip8Error> {
        let pc = self.state().program_counter();
        let hi = self.state().memory(pc)?;
        let lo = self.state().memory(pc + 1)?;

        self.state().increment_program_counter();
        let opcode = u16::from_be_bytes([hi, lo]);

        let curr_row = &mut self.state().cpu_trace.curr_row;
        curr_row.opcode = F::from_canonical_u16(opcode);

        Ok(opcode)
    }

    fn decode(&mut self, opcode: u16) -> Result<Instruction, Chip8Error> {
        let curr_row = &mut self.state().cpu_trace.curr_row;

        let x = ((opcode >> 8) & 0x000F) as u8;
        let y = ((opcode >> 4) & 0x000F) as u8;

        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        curr_row.x = F::from_canonical_u8(x);
        curr_row.y = F::from_canonical_u8(y);

        for i in 0..NUM_REGISTERS {
            curr_row.x_sel[i] = F::from_bool(x == i as u8);
            curr_row.y_sel[i] = F::from_bool(y == i as u8);
            curr_row.lte_x_sel[i] = F::from_bool((i as u8) <= x);
        }

        curr_row.n = F::from_canonical_u8(n);
        curr_row.nn = F::from_canonical_u8(nn);
        curr_row.nnn = F::from_canonical_u16(nnn);

        // TODO: Constraints for this
        match opcode & 0xF000 {
            0x0000 => match opcode & 0xF0FF {
                // 0x00E0
                0x00E0 => {
                    curr_row.is_clear_display = F::one();
                    Ok(Instruction::ClearDisplay)
                }
                // 0x00EE
                0x00EE => {
                    curr_row.is_return = F::one();
                    Ok(Instruction::Return)
                }
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            // 0x1NNN
            0x1000 => {
                curr_row.is_jump = F::one();
                Ok(Instruction::Jump(nnn))
            }
            // 0x2NNN
            0x2000 => {
                curr_row.is_call = F::one();
                Ok(Instruction::Call(nnn))
            }
            // 0x3XNN
            0x3000 => {
                curr_row.is_skip_equal = F::one();
                Ok(Instruction::SkipEqual(x, nn))
            }
            // 0x4XNN
            0x4000 => {
                curr_row.is_skip_not_equal = F::one();
                Ok(Instruction::SkipNotEqual(x, nn))
            }
            // 0x5XY0
            0x5000 => match opcode & 0xF00F {
                0x5000 => {
                    curr_row.is_skip_equal_xy = F::one();
                    Ok(Instruction::SkipEqualXY(x, y))
                }
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            // 0x6XNN
            0x6000 => {
                curr_row.is_load = F::one();
                Ok(Instruction::Load(x, nn))
            }
            // 0x7XNN
            0x7000 => {
                curr_row.is_add = F::one();
                Ok(Instruction::Add(x, nn))
            }
            0x8000 => match opcode & 0xF00F {
                // 0x8XY0
                0x8000 => {
                    curr_row.is_move = F::one();
                    Ok(Instruction::Move(x, y))
                }
                // 0x8XY1
                0x8001 => {
                    curr_row.is_or = F::one();
                    Ok(Instruction::Or(x, y))
                }
                // 0x8XY2
                0x8002 => {
                    curr_row.is_and = F::one();
                    Ok(Instruction::And(x, y))
                }
                // 0x8XY3
                0x8003 => {
                    curr_row.is_xor = F::one();
                    Ok(Instruction::Xor(x, y))
                }
                // 0x8XY4
                0x8004 => {
                    curr_row.is_add_xy = F::one();
                    Ok(Instruction::AddXY(x, y))
                }
                // 0x8XY5
                0x8005 => {
                    curr_row.is_sub_xy = F::one();
                    Ok(Instruction::SubXY(x, y))
                }
                // 0x8XY6
                0x8006 => {
                    curr_row.is_shift_right = F::one();
                    Ok(Instruction::ShiftRight(x))
                }
                // 0x8XY7
                0x8007 => {
                    curr_row.is_sub_yx = F::one();
                    Ok(Instruction::SubYX(x, y))
                }
                // 0x8XYE
                0x800E => {
                    curr_row.is_shift_left = F::one();
                    Ok(Instruction::ShiftLeft(x))
                }
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            0x9000 => match opcode & 0xF00F {
                // 0x9XY0
                0x9000 => {
                    curr_row.is_skip_not_equal_xy = F::one();
                    Ok(Instruction::SkipNotEqualXY(x, y))
                }
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            // 0xANNN
            0xA000 => {
                curr_row.is_load_i = F::one();
                Ok(Instruction::LoadI(nnn))
            }
            // 0xBNNN
            0xB000 => {
                curr_row.is_jump_v0 = F::one();
                Ok(Instruction::JumpV0(nnn))
            }
            // 0xCXNN
            0xC000 => {
                curr_row.is_random = F::one();
                Ok(Instruction::Random(x, nn))
            }
            // 0xDXYN
            0xD000 => {
                curr_row.is_draw = F::one();
                Ok(Instruction::Draw(x, y, n))
            }
            0xE000 => match opcode & 0xF0FF {
                // 0xEX9E
                0xE09E => {
                    curr_row.is_skip_key_pressed = F::one();
                    Ok(Instruction::SkipKeyPressed(x))
                }
                // 0xEXA1
                0xE0A1 => {
                    curr_row.is_skip_key_not_pressed = F::one();
                    Ok(Instruction::SkipKeyNotPressed(x))
                }
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            0xF000 => match opcode & 0xF0FF {
                // 0xFX07
                0xF007 => {
                    curr_row.is_load_delay = F::one();
                    Ok(Instruction::LoadDelay(x))
                }
                // 0xFX0A
                0xF00A => {
                    curr_row.is_wait_key_press = F::one();
                    Ok(Instruction::WaitKeyPress(x))
                }
                // 0xFX15
                0xF015 => {
                    curr_row.is_set_delay = F::one();
                    Ok(Instruction::SetDelay(x))
                }
                // 0xFX18
                0xF018 => {
                    curr_row.is_set_sound = F::one();
                    Ok(Instruction::SetSound(x))
                }
                // 0xFX1E
                0xF01E => {
                    curr_row.is_add_i = F::one();
                    Ok(Instruction::AddI(x))
                }
                // 0xFX29
                0xF029 => {
                    curr_row.is_load_font = F::one();
                    Ok(Instruction::LoadFont(x))
                }
                // 0xFX33
                0xF033 => {
                    curr_row.is_store_bcd = F::one();
                    Ok(Instruction::StoreBCD(x))
                }
                // 0xFX55
                0xF055 => {
                    curr_row.is_store_registers = F::one();
                    Ok(Instruction::StoreRegisters(x))
                }
                // 0xFX65
                0xF065 => {
                    curr_row.is_load_memory = F::one();
                    Ok(Instruction::LoadMemory(x))
                }
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
        }
    }
}
