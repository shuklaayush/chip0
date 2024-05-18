use chip8_core::{
    constants::{NUM_REGISTERS, TICKS_PER_TIMER},
    cpu::Cpu,
    drivers::ProofRequest,
    error::Chip8Error,
    input::{InputEvent, InputQueue},
    instruction::Instruction,
    rwlock::CheckedWrite,
    state::{State, Word},
    util::run_loop,
};
use p3_field::{AbstractField, PrimeField32};
use p3_uni_stark::{StarkGenericConfig, Val};
use rand::Rng;
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use crate::trace::StarkState;

pub const TICKS_PER_PROOF: u64 = 5;

pub struct StarkCpu<R, SC>
where
    R: Rng,
    SC: StarkGenericConfig,
    Val<SC>: PrimeField32,
{
    state: StarkState<Val<SC>>,
    clk_freq: u64,
    rng: R,
}

impl<R, SC> StarkCpu<R, SC>
where
    R: Rng,
    SC: StarkGenericConfig,
    Val<SC>: PrimeField32,
{
    pub fn new(clk_freq: u64, rng: R) -> Self {
        Self {
            state: StarkState::default(),
            clk_freq,
            rng,
        }
    }
}

impl<R, SC> Cpu<SC> for StarkCpu<R, SC>
where
    R: Rng,
    SC: StarkGenericConfig,
    Val<SC>: PrimeField32,
{
    type State = StarkState<Val<SC>>;

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
        curr_row.opcode = Val::<SC>::from_canonical_u16(opcode);

        Ok(opcode)
    }

    fn decode(&mut self, opcode: u16) -> Result<Instruction, Chip8Error> {
        let curr_row = &mut self.state().cpu_trace.curr_row;

        let x = ((opcode >> 8) & 0x000F) as u8;
        let y = ((opcode >> 4) & 0x000F) as u8;

        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        curr_row.x = Val::<SC>::from_canonical_u8(x);
        curr_row.y = Val::<SC>::from_canonical_u8(y);

        for i in 0..NUM_REGISTERS {
            curr_row.x_sel[i] = Val::<SC>::from_bool(x == i as u8);
            curr_row.y_sel[i] = Val::<SC>::from_bool(y == i as u8);
            curr_row.lte_x_sel[i] = Val::<SC>::from_bool((i as u8) <= x);
        }

        curr_row.n = Val::<SC>::from_canonical_u8(n);
        curr_row.nn = Val::<SC>::from_canonical_u8(nn);
        curr_row.nnn = Val::<SC>::from_canonical_u16(nnn);

        // TODO: Constraints for this
        match opcode & 0xF000 {
            0x0000 => match opcode & 0xF0FF {
                // 0x00E0
                0x00E0 => {
                    curr_row.is_clear_display = Val::<SC>::one();
                    Ok(Instruction::ClearDisplay)
                }
                // 0x00EE
                0x00EE => {
                    curr_row.is_return = Val::<SC>::one();
                    Ok(Instruction::Return)
                }
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            // 0x1NNN
            0x1000 => {
                curr_row.is_jump = Val::<SC>::one();
                Ok(Instruction::Jump(nnn))
            }
            // 0x2NNN
            0x2000 => {
                curr_row.is_call = Val::<SC>::one();
                Ok(Instruction::Call(nnn))
            }
            // 0x3XNN
            0x3000 => {
                curr_row.is_skip_equal = Val::<SC>::one();
                Ok(Instruction::SkipEqual(x, nn))
            }
            // 0x4XNN
            0x4000 => {
                curr_row.is_skip_not_equal = Val::<SC>::one();
                Ok(Instruction::SkipNotEqual(x, nn))
            }
            // 0x5XY0
            0x5000 => match opcode & 0xF00F {
                0x5000 => {
                    curr_row.is_skip_equal_xy = Val::<SC>::one();
                    Ok(Instruction::SkipEqualXY(x, y))
                }
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            // 0x6XNN
            0x6000 => {
                curr_row.is_load = Val::<SC>::one();
                Ok(Instruction::Load(x, nn))
            }
            // 0x7XNN
            0x7000 => {
                curr_row.is_add = Val::<SC>::one();
                Ok(Instruction::Add(x, nn))
            }
            0x8000 => match opcode & 0xF00F {
                // 0x8XY0
                0x8000 => {
                    curr_row.is_move = Val::<SC>::one();
                    Ok(Instruction::Move(x, y))
                }
                // 0x8XY1
                0x8001 => {
                    curr_row.is_or = Val::<SC>::one();
                    Ok(Instruction::Or(x, y))
                }
                // 0x8XY2
                0x8002 => {
                    curr_row.is_and = Val::<SC>::one();
                    Ok(Instruction::And(x, y))
                }
                // 0x8XY3
                0x8003 => {
                    curr_row.is_xor = Val::<SC>::one();
                    Ok(Instruction::Xor(x, y))
                }
                // 0x8XY4
                0x8004 => {
                    curr_row.is_add_xy = Val::<SC>::one();
                    Ok(Instruction::AddXY(x, y))
                }
                // 0x8XY5
                0x8005 => {
                    curr_row.is_sub_xy = Val::<SC>::one();
                    Ok(Instruction::SubXY(x, y))
                }
                // 0x8XY6
                0x8006 => {
                    curr_row.is_shift_right = Val::<SC>::one();
                    Ok(Instruction::ShiftRight(x))
                }
                // 0x8XY7
                0x8007 => {
                    curr_row.is_sub_yx = Val::<SC>::one();
                    Ok(Instruction::SubYX(x, y))
                }
                // 0x8XYE
                0x800E => {
                    curr_row.is_shift_left = Val::<SC>::one();
                    Ok(Instruction::ShiftLeft(x))
                }
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            0x9000 => match opcode & 0xF00F {
                // 0x9XY0
                0x9000 => {
                    curr_row.is_skip_not_equal_xy = Val::<SC>::one();
                    Ok(Instruction::SkipNotEqualXY(x, y))
                }
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            // 0xANNN
            0xA000 => {
                curr_row.is_load_i = Val::<SC>::one();
                Ok(Instruction::LoadI(nnn))
            }
            // 0xBNNN
            0xB000 => {
                curr_row.is_jump_v0 = Val::<SC>::one();
                Ok(Instruction::JumpV0(nnn))
            }
            // 0xCXNN
            0xC000 => {
                curr_row.is_random = Val::<SC>::one();
                Ok(Instruction::Random(x, nn))
            }
            // 0xDXYN
            0xD000 => {
                curr_row.is_draw = Val::<SC>::one();
                Ok(Instruction::Draw(x, y, n))
            }
            0xE000 => match opcode & 0xF0FF {
                // 0xEX9E
                0xE09E => {
                    curr_row.is_skip_key_pressed = Val::<SC>::one();
                    Ok(Instruction::SkipKeyPressed(x))
                }
                // 0xEXA1
                0xE0A1 => {
                    curr_row.is_skip_key_not_pressed = Val::<SC>::one();
                    Ok(Instruction::SkipKeyNotPressed(x))
                }
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            0xF000 => match opcode & 0xF0FF {
                // 0xFX07
                0xF007 => {
                    curr_row.is_load_delay = Val::<SC>::one();
                    Ok(Instruction::LoadDelay(x))
                }
                // 0xFX0A
                0xF00A => {
                    curr_row.is_wait_key_press = Val::<SC>::one();
                    Ok(Instruction::WaitKeyPress(x))
                }
                // 0xFX15
                0xF015 => {
                    curr_row.is_set_delay = Val::<SC>::one();
                    Ok(Instruction::SetDelay(x))
                }
                // 0xFX18
                0xF018 => {
                    curr_row.is_set_sound = Val::<SC>::one();
                    Ok(Instruction::SetSound(x))
                }
                // 0xFX1E
                0xF01E => {
                    curr_row.is_add_i = Val::<SC>::one();
                    Ok(Instruction::AddI(x))
                }
                // 0xFX29
                0xF029 => {
                    curr_row.is_load_font = Val::<SC>::one();
                    Ok(Instruction::LoadFont(x))
                }
                // 0xFX33
                0xF033 => {
                    curr_row.is_store_bcd = Val::<SC>::one();
                    Ok(Instruction::StoreBCD(x))
                }
                // 0xFX55
                0xF055 => {
                    curr_row.is_store_registers = Val::<SC>::one();
                    Ok(Instruction::StoreRegisters(x))
                }
                // 0xFX65
                0xF065 => {
                    curr_row.is_load_memory = Val::<SC>::one();
                    Ok(Instruction::LoadMemory(x))
                }
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
        }
    }

    fn run(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        input_queue: Arc<RwLock<VecDeque<(u64, InputEvent)>>>,
        proving_queue: Arc<RwLock<VecDeque<ProofRequest<Val<SC>>>>>,
    ) {
        run_loop(status.clone(), self.frequency(), move |_| {
            let clk = self.state().clk()?;

            let curr_row = &mut self.state().cpu_trace.curr_row;
            if clk == 0 {
                curr_row.is_first = Val::<SC>::one();
            }
            curr_row.is_real = Val::<SC>::one();

            while let Some(event) = (*input_queue.checked_write()?).dequeue(clk) {
                self.state().set_key(event.key, event.kind);
            }
            // TODO: How do I remove this clone?
            self.tick(status.clone(), input_queue.clone())?;
            if clk % TICKS_PER_TIMER == 0 {
                self.tick_timers()?;
            }

            self.state().increment_clk()?;

            if (clk + 1) % TICKS_PER_PROOF == 0 {
                // TODO: Generate trace in other thread
                let request = ProofRequest {
                    traces: self.state.get_trace_matrices(),
                    // TODO
                    public_values: vec![],
                };
                proving_queue.checked_write()?.push_back(request);
                self.op_wait_key_press(status.clone(), input_queue.clone(), 0)?;
            }

            Ok(())
        });
    }
}
