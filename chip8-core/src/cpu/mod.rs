use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

mod simple;
pub use simple::SimpleCpu;

use crate::{
    constants::{
        DISPLAY_HEIGHT, DISPLAY_WIDTH, FONTSET_START_ADDRESS, FONT_SIZE, NUM_KEYS, TICKS_PER_TIMER,
    },
    error::Chip8Error,
    input::{InputEvent, InputQueue},
    instruction::Instruction,
    rwlock::{CheckedRead, CheckedWrite},
    state::{Address, State, Word},
    util::run_loop,
};

pub trait Cpu {
    type State: State;

    fn state(&mut self) -> &mut Self::State;

    fn frequency(&self) -> u64;

    fn random(&mut self) -> Word;

    // Instructions
    fn op_clear_display(&mut self) -> Result<(), Chip8Error> {
        self.state().clear_framebuffer()
    }

    fn op_return(&mut self) {
        self.state().pop_stack()
    }

    fn op_jump(&mut self, nnn: Address) {
        self.state().set_program_counter(nnn);
    }

    fn op_call(&mut self, nnn: Address) {
        self.state().push_stack(nnn);
    }

    fn op_skip_equal(&mut self, x: Word, nn: Word) {
        let vx = self.state().register(x);
        if vx == nn {
            self.state().increment_program_counter();
        }
    }

    fn op_skip_not_equal(&mut self, x: Word, nn: Word) {
        let vx = self.state().register(x);
        if vx != nn {
            self.state().increment_program_counter();
        }
    }

    fn op_skip_equal_xy(&mut self, x: Word, y: Word) {
        let vx = self.state().register(x);
        let vy = self.state().register(y);
        if vx == vy {
            self.state().increment_program_counter();
        }
    }

    fn op_load(&mut self, x: Word, nn: Word) {
        self.state().set_register(x, nn);
    }

    fn op_add(&mut self, x: Word, nn: Word) {
        let vx = self.state().register(x);
        let val = vx.wrapping_add(nn);
        self.state().set_register(x, val);
    }

    fn op_move(&mut self, x: Word, y: Word) {
        let vy = self.state().register(y);
        self.state().set_register(x, vy);
    }

    fn op_or(&mut self, x: Word, y: Word) {
        let vx = self.state().register(x);
        let vy = self.state().register(y);
        let val = vx | vy;
        self.state().set_register(x, val);
    }

    fn op_and(&mut self, x: Word, y: Word) {
        let vx = self.state().register(x);
        let vy = self.state().register(y);
        let val = vx & vy;
        self.state().set_register(x, val);
    }

    fn op_xor(&mut self, x: Word, y: Word) {
        let vx = self.state().register(x);
        let vy = self.state().register(y);
        let val = vx ^ vy;
        self.state().set_register(x, val);
    }

    fn op_add_xy(&mut self, x: Word, y: Word) {
        let vx = self.state().register(x);
        let vy = self.state().register(y);
        let (sum, carry) = vx.overflowing_add(vy);

        self.state().set_register(x, sum);
        self.state().set_flag_register(carry);
    }

    fn op_sub_xy(&mut self, x: Word, y: Word) {
        let vx = self.state().register(x);
        let vy = self.state().register(y);
        let (diff, borrow) = vx.overflowing_sub(vy);

        self.state().set_register(x, diff);
        self.state().set_flag_register(!borrow);
    }

    fn op_shift_right(&mut self, x: Word) {
        let vx = self.state().register(x);
        let flag = (vx & 1) != 0;
        let val = vx >> 1;

        self.state().set_flag_register(flag);
        self.state().set_register(x, val);
    }

    fn op_sub_yx(&mut self, x: Word, y: Word) {
        let vx = self.state().register(x);
        let vy = self.state().register(y);
        let (diff, borrow) = vy.overflowing_sub(vx);

        self.state().set_register(x, diff);
        self.state().set_flag_register(!borrow);
    }

    fn op_shift_left(&mut self, x: Word) {
        let vx = self.state().register(x);
        let flag = ((vx >> 7) & 1) != 0;
        let val = vx << 1;

        self.state().set_register(x, val);
        self.state().set_flag_register(flag);
    }

    fn op_skip_not_equal_xy(&mut self, x: Word, y: Word) {
        let vx = self.state().register(x);
        let vy = self.state().register(y);
        if vx != vy {
            self.state().increment_program_counter();
        }
    }

    fn op_load_i(&mut self, nnn: Address) {
        self.state().set_index_register(nnn);
    }

    fn op_jump_v0(&mut self, nnn: Address) {
        let v0 = self.state().register(0);
        let offset = (v0 as u16) + nnn;
        self.state().set_program_counter(offset);
    }

    fn op_random(&mut self, x: Word, nn: Word) {
        let r = self.random();
        let val = r & nn;
        self.state().set_register(x, val);
    }

    fn op_draw(&mut self, x: Word, y: Word, n: Word) -> Result<(), Chip8Error> {
        let vx = self.state().register(x);
        let vy = self.state().register(y);
        let vi = self.state().index_register();

        let x0 = vx as usize % DISPLAY_WIDTH;
        let y0 = vy as usize % DISPLAY_HEIGHT;
        let mut flipped = false;
        for ys in 0..n {
            let y = (y0 + ys as usize) % DISPLAY_HEIGHT;
            let pixels = self.state().memory(vi + ys as u16)?;
            for xs in 0..8 {
                let x = (x0 + xs) % DISPLAY_WIDTH;
                let pixel = (pixels >> (7 - xs)) & 1 == 1;
                let fb = self.state().frame_buffer(y, x)?;
                flipped |= pixel & fb;
                if pixel {
                    self.state().set_frame_buffer(y, x, !fb)?;
                }
            }
        }
        self.state().set_flag_register(flipped);
        Ok(())
    }

    fn op_skip_key_pressed(&mut self, x: Word) {
        let vx = self.state().register(x);
        if self.state().key(vx) {
            self.state().increment_program_counter();
        }
    }

    fn op_skip_key_not_pressed(&mut self, x: Word) {
        let vx = self.state().register(x);
        if !self.state().key(vx) {
            self.state().increment_program_counter();
        }
    }

    fn op_load_delay(&mut self, x: Word) {
        let val = self.state().delay_timer();
        self.state().set_register(x, val);
    }

    fn op_wait_key_press(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        input_queue: Arc<RwLock<VecDeque<(u64, InputEvent)>>>,
        x: Word,
    ) -> Result<(), Chip8Error> {
        let clk = self.state().clk()?;

        let mut pressed = false;
        for i in 0..NUM_KEYS {
            if self.state().key(i as u8) {
                pressed = true;
            }
        }
        while status.checked_read()?.is_ok() && !pressed {
            if let Some(event) = (*input_queue.checked_write()?).dequeue(clk) {
                self.state().set_key(event.key, event.kind);
                self.state().set_register(x, event.key as u8);
                pressed = true;
            }
        }
        Ok(())
    }

    fn op_set_delay(&mut self, x: Word) {
        let vx = self.state().register(x);
        self.state().set_delay_timer(vx);
    }

    fn op_set_sound(&mut self, x: Word) -> Result<(), Chip8Error> {
        let vx = self.state().register(x);
        self.state().set_sound_timer(vx)
    }

    fn op_add_i(&mut self, x: Word) {
        let vx = self.state().register(x);
        let vi = self.state().index_register();
        let addr = vi.wrapping_add(vx as u16);
        self.state().set_index_register(addr);
    }

    fn op_load_font(&mut self, x: Word) {
        let vx = self.state().register(x);
        let addr = FONTSET_START_ADDRESS + (FONT_SIZE as u16) * (vx as u16);
        self.state().set_index_register(addr);
    }

    fn op_store_bcd(&mut self, x: Word) -> Result<(), Chip8Error> {
        let vx = self.state().register(x);
        let vi = self.state().index_register();

        self.state().set_memory(vi, (vx / 100) % 10)?;
        self.state().set_memory(vi + 1, (vx / 10) % 10)?;
        self.state().set_memory(vi + 2, vx % 10)
    }

    fn op_store_registers(&mut self, x: Word) -> Result<(), Chip8Error> {
        let vi = self.state().index_register();
        for j in 0..=x {
            let vj = self.state().register(j);
            self.state().set_memory(vi + j as u16, vj)?;
        }
        Ok(())
    }

    fn op_load_memory(&mut self, x: Word) -> Result<(), Chip8Error> {
        let vi = self.state().index_register();
        for j in 0..=x {
            let val = self.state().memory(vi + j as u16)?;
            self.state().set_register(j, val);
        }
        Ok(())
    }

    // Fetch - Decode - Execute
    fn fetch(&mut self) -> Result<u16, Chip8Error> {
        let pc = self.state().program_counter();
        let hi = self.state().memory(pc)?;
        let lo = self.state().memory(pc + 1)?;

        self.state().increment_program_counter();
        Ok(u16::from_be_bytes([hi, lo]))
    }

    fn decode(&mut self, opcode: u16) -> Result<Instruction, Chip8Error> {
        let x = ((opcode >> 8) & 0x000F) as u8;
        let y = ((opcode >> 4) & 0x000F) as u8;

        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match opcode & 0xF000 {
            0x0000 => match opcode & 0xF0FF {
                // 0x00E0
                0x00E0 => Ok(Instruction::ClearDisplay),
                // 0x00EE
                0x00EE => Ok(Instruction::Return),
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            // 0x1NNN
            0x1000 => Ok(Instruction::Jump(nnn)),
            // 0x2NNN
            0x2000 => Ok(Instruction::Call(nnn)),
            // 0x3XNN
            0x3000 => Ok(Instruction::SkipEqual(x, nn)),
            // 0x4XNN
            0x4000 => Ok(Instruction::SkipNotEqual(x, nn)),
            // 0x5XY0
            0x5000 => match opcode & 0xF00F {
                0x5000 => Ok(Instruction::SkipEqualXY(x, y)),
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            // 0x6XNN
            0x6000 => Ok(Instruction::Load(x, nn)),
            // 0x7XNN
            0x7000 => Ok(Instruction::Add(x, nn)),
            0x8000 => match opcode & 0xF00F {
                // 0x8XY0
                0x8000 => Ok(Instruction::Move(x, y)),
                // 0x8XY1
                0x8001 => Ok(Instruction::Or(x, y)),
                // 0x8XY2
                0x8002 => Ok(Instruction::And(x, y)),
                // 0x8XY3
                0x8003 => Ok(Instruction::Xor(x, y)),
                // 0x8XY4
                0x8004 => Ok(Instruction::AddXY(x, y)),
                // 0x8XY5
                0x8005 => Ok(Instruction::SubXY(x, y)),
                // 0x8XY6
                0x8006 => Ok(Instruction::ShiftRight(x)),
                // 0x8XY7
                0x8007 => Ok(Instruction::SubYX(x, y)),
                // 0x8XYE
                0x800E => Ok(Instruction::ShiftLeft(x)),
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            0x9000 => match opcode & 0xF00F {
                // 0x9XY0
                0x9000 => Ok(Instruction::SkipNotEqualXY(x, y)),
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            // 0xANNN
            0xA000 => Ok(Instruction::LoadI(nnn)),
            // 0xBNNN
            0xB000 => Ok(Instruction::JumpV0(nnn)),
            // 0xCXNN
            0xC000 => Ok(Instruction::Random(x, nn)),
            // 0xDXYN
            0xD000 => Ok(Instruction::Draw(x, y, n)),
            0xE000 => match opcode & 0xF0FF {
                // 0xEX9E
                0xE09E => Ok(Instruction::SkipKeyPressed(x)),
                // 0xEXA1
                0xE0A1 => Ok(Instruction::SkipKeyNotPressed(x)),
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            0xF000 => match opcode & 0xF0FF {
                // 0xFX07
                0xF007 => Ok(Instruction::LoadDelay(x)),
                // 0xFX0A
                0xF00A => Ok(Instruction::WaitKeyPress(x)),
                // 0xFX15
                0xF015 => Ok(Instruction::SetDelay(x)),
                // 0xFX18
                0xF018 => Ok(Instruction::SetSound(x)),
                // 0xFX1E
                0xF01E => Ok(Instruction::AddI(x)),
                // 0xFX29
                0xF029 => Ok(Instruction::LoadFont(x)),
                // 0xFX33
                0xF033 => Ok(Instruction::StoreBCD(x)),
                // 0xFX55
                0xF055 => Ok(Instruction::StoreRegisters(x)),
                // 0xFX65
                0xF065 => Ok(Instruction::LoadMemory(x)),
                _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
            },
            _ => Err(Chip8Error::UnimplementedOpcode(opcode)),
        }
    }

    fn execute(
        &mut self,
        instruction: Instruction,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        input_queue: Arc<RwLock<VecDeque<(u64, InputEvent)>>>,
    ) -> Result<(), Chip8Error> {
        match instruction {
            Instruction::ClearDisplay => {
                self.op_clear_display()?;
            }
            Instruction::Return => {
                self.op_return();
            }
            Instruction::Jump(nnn) => {
                self.op_jump(nnn);
            }
            Instruction::Call(nnn) => {
                self.op_call(nnn);
            }
            Instruction::SkipEqual(x, nn) => {
                self.op_skip_equal(x, nn);
            }
            Instruction::SkipNotEqual(x, nn) => {
                self.op_skip_not_equal(x, nn);
            }
            Instruction::SkipEqualXY(x, y) => {
                self.op_skip_equal_xy(x, y);
            }
            Instruction::Load(x, nn) => {
                self.op_load(x, nn);
            }
            Instruction::Add(x, nn) => {
                self.op_add(x, nn);
            }
            Instruction::Move(x, y) => {
                self.op_move(x, y);
            }
            Instruction::Or(x, y) => {
                self.op_or(x, y);
            }
            Instruction::And(x, y) => {
                self.op_and(x, y);
            }
            Instruction::Xor(x, y) => {
                self.op_xor(x, y);
            }
            Instruction::AddXY(x, y) => {
                self.op_add_xy(x, y);
            }
            Instruction::SubXY(x, y) => {
                self.op_sub_xy(x, y);
            }
            Instruction::ShiftRight(x) => {
                self.op_shift_right(x);
            }
            Instruction::SubYX(x, y) => {
                self.op_sub_yx(x, y);
            }
            Instruction::ShiftLeft(x) => {
                self.op_shift_left(x);
            }
            Instruction::SkipNotEqualXY(x, y) => {
                self.op_skip_not_equal_xy(x, y);
            }
            Instruction::LoadI(nnn) => {
                self.op_load_i(nnn);
            }
            Instruction::JumpV0(nnn) => {
                self.op_jump_v0(nnn);
            }
            Instruction::Random(x, nn) => {
                self.op_random(x, nn);
            }
            Instruction::Draw(x, y, n) => {
                self.op_draw(x, y, n)?;
            }
            Instruction::SkipKeyPressed(x) => {
                self.op_skip_key_pressed(x);
            }
            Instruction::SkipKeyNotPressed(x) => {
                self.op_skip_key_not_pressed(x);
            }
            Instruction::LoadDelay(x) => {
                self.op_load_delay(x);
            }
            Instruction::WaitKeyPress(x) => {
                self.op_wait_key_press(status, input_queue, x)?;
            }
            Instruction::SetDelay(x) => {
                self.op_set_delay(x);
            }
            Instruction::SetSound(x) => {
                self.op_set_sound(x)?;
            }
            Instruction::AddI(x) => {
                self.op_add_i(x);
            }
            Instruction::LoadFont(x) => {
                self.op_load_font(x);
            }
            Instruction::StoreBCD(x) => {
                self.op_store_bcd(x)?;
            }
            Instruction::StoreRegisters(x) => {
                self.op_store_registers(x)?;
            }
            Instruction::LoadMemory(x) => {
                self.op_load_memory(x)?;
            }
        }

        Ok(())
    }

    // Cycle
    fn tick(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        input_queue: Arc<RwLock<VecDeque<(u64, InputEvent)>>>,
    ) -> Result<(), Chip8Error> {
        let op = self.fetch()?;
        let instruction = self.decode(op)?;
        self.execute(instruction, status, input_queue)
    }

    fn tick_timers(&mut self) -> Result<(), Chip8Error> {
        if self.state().delay_timer() > 0 {
            self.state().decrement_delay_timer();
        }
        if self.state().sound_timer()? > 0 {
            self.state().decrement_sound_timer()?;
        }
        Ok(())
    }

    async fn run(
        &mut self,
        status: Arc<RwLock<Result<(), Chip8Error>>>,
        input_queue: Arc<RwLock<VecDeque<(u64, InputEvent)>>>,
    ) {
        run_loop(status.clone(), self.frequency(), move |_| {
            let clk = self.state().clk()?;

            while let Some(event) = (*input_queue.checked_write()?).dequeue(clk) {
                self.state().set_key(event.key, event.kind);
            }
            // TODO: How do I remove this clone?
            self.tick(status.clone(), input_queue.clone())?;
            if clk % TICKS_PER_TIMER == 0 {
                self.tick_timers()?;
            }

            self.state().increment_clk()?;
            Ok(())
        })
    }
}
