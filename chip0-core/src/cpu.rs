use chip8_core::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH, NUM_REGISTERS, TICKS_PER_TIMER},
    cpu::Cpu,
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

use crate::{prover::Prover, trace::StarkState};

pub const TICKS_PER_PROOF: u64 = 1000;

pub struct StarkCpu<R, SC, P>
where
    R: Rng,
    SC: StarkGenericConfig,
    Val<SC>: PrimeField32,
    P: Prover<SC> + Send + Sync + 'static,
{
    state: StarkState<Val<SC>>,
    clk_freq: u64,
    rng: R,

    prover: Arc<P>,
}

impl<R, SC, P> StarkCpu<R, SC, P>
where
    R: Rng,
    SC: StarkGenericConfig,
    Val<SC>: PrimeField32,
    P: Prover<SC> + Send + Sync + 'static,
{
    pub fn new(clk_freq: u64, rng: R, prover: P) -> Self {
        Self {
            state: StarkState::default(),
            clk_freq,
            rng,
            prover: Arc::new(prover),
        }
    }
}

impl<R, SC, P> Cpu for StarkCpu<R, SC, P>
where
    R: Rng,
    SC: StarkGenericConfig,
    Val<SC>: PrimeField32,
    P: Prover<SC> + Send + Sync + 'static,
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

    fn op_draw(&mut self, x: Word, y: Word, n: Word) -> Result<(), Chip8Error> {
        let clk = self.state().clk()?;

        let vx = self.state().register(x);
        let vy = self.state().register(y);
        let vi = self.state().index_register();

        let x0 = vx as usize % DISPLAY_WIDTH;
        let y0 = vy as usize % DISPLAY_HEIGHT;
        let mut flipped = false;

        // Each row in loop
        let curr_row = &mut self.state().trace.draw.curr_row;
        curr_row.is_real = Val::<SC>::one();
        curr_row.clk = Val::<SC>::from_canonical_u64(clk);
        curr_row.register_x = Val::<SC>::from_canonical_u8(vx);
        curr_row.register_y = Val::<SC>::from_canonical_u8(vy);
        curr_row.index_register = Val::<SC>::from_canonical_u16(vi);

        for ys in 0..n {
            let y = (y0 + ys as usize) % DISPLAY_HEIGHT;
            let pixels = self.state().memory(vi + ys as u16)?;
            for xs in 0..8u8 {
                let x = (x0 + xs as usize) % DISPLAY_WIDTH;
                let pixel = (pixels >> (7 - xs)) & 1 == 1;
                let fb = self.state().frame_buffer(y, x)?;
                let curr_flipped = pixel & fb;
                flipped |= curr_flipped;
                if pixel {
                    self.state().set_frame_buffer(y, x, !fb)?;
                }

                let curr_row = &mut self.state().trace.draw.curr_row;
                curr_row.ys = Val::<SC>::from_canonical_u8(ys);
                curr_row.y = Val::<SC>::from_canonical_usize(y);
                curr_row.pixels = Val::<SC>::from_canonical_u8(pixels);
                curr_row.xs = Val::<SC>::from_canonical_u8(xs);
                curr_row.x = Val::<SC>::from_canonical_usize(x);
                curr_row.pixel = Val::<SC>::from_bool(pixel);
                curr_row.frame_buffer_y_x = Val::<SC>::from_bool(fb);
                curr_row.flipped = Val::<SC>::from_bool(curr_flipped);
                curr_row.register_flag = Val::<SC>::from_bool(flipped);

                self.state().trace.draw.add_curr_row_to_trace();
            }
        }
        self.state().set_flag_register(flipped);
        Ok(())
    }

    fn fetch(&mut self) -> Result<u16, Chip8Error> {
        let pc = self.state().program_counter();
        let hi = self.state().memory(pc)?;
        let lo = self.state().memory(pc + 1)?;

        self.state().increment_program_counter();
        let opcode = u16::from_be_bytes([hi, lo]);

        let curr_row = &mut self.state().trace.cpu.curr_row;
        curr_row.opcode = Val::<SC>::from_canonical_u16(opcode);

        Ok(opcode)
    }

    fn decode(&mut self, opcode: u16) -> Result<Instruction, Chip8Error> {
        let curr_row = &mut self.state().trace.cpu.curr_row;

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

    async fn run(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        input_queue: Arc<RwLock<VecDeque<(u64, InputEvent)>>>,
    ) {
        // let mut prover_handle = None;
        run_loop(status.clone(), self.frequency(), move |_| {
            let clk = self.state().clk()?;

            let curr_row = &mut self.state().trace.cpu.curr_row;
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
                let trace = self.state.finalize_trace();
                let prover = self.prover.clone();
                tokio::spawn(async move { prover.prove(trace) });

                self.op_wait_key_press(status.clone(), input_queue.clone(), 0)?;
            }

            Ok(())
        });
        // if let Some(prover_handle) = prover_handle {
        //     prover_handle
        //         .await
        //         .map_err(|e| Chip8Error::AsyncAwaitError(e.to_string()));
        // }
    }
}
