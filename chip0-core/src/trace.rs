use chip8_core::{
    constants::{
        DISPLAY_HEIGHT, DISPLAY_WIDTH, FLAG_REGISTER, MEMORY_SIZE, NUM_KEYS, OPCODE_SIZE,
        PROGRAM_START_ADDRESS, STACK_DEPTH,
    },
    error::Chip8Error,
    input::InputKind,
    keypad::Key,
    state::{Address, SimpleState, State, Word},
};
use core::slice;
use itertools::Itertools;
use p3_field::PrimeField32;
use p3_matrix::dense::RowMajorMatrix;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, RwLock},
};

use crate::chips::{
    clear::columns::ClearCols, cpu::columns::CpuCols, draw::columns::DrawCols,
    frame_buffer::columns::FrameBufferCols, keypad::columns::KeypadCols,
    memory::columns::MemoryCols, memory_start::columns::MemoryStartCols, range::columns::RangeCols,
};

#[derive(Default, Clone)]
pub struct IncrementalTrace<Cols: Default> {
    pub trace: Vec<Cols>,
    pub curr_row: Cols,
    pub next_row: Cols,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct MemoryEventLike<T> {
    pub clk: T,
    pub address: T,
    pub value: T,
    pub is_read: T,
}

#[derive(Clone)]
pub struct PartialMachineTrace<F: PrimeField32> {
    pub cpu: Vec<CpuCols<F>>,
    pub clear: Vec<ClearCols<F>>,
    pub draw: Vec<DrawCols<F>>,
    pub keypad: Vec<KeypadCols<F>>,
    // range_trace: Vec::default(),
    pub memory: Vec<MemoryEventLike<F>>,
    pub frame_buffer: Vec<MemoryEventLike<F>>,
    // TODO: Change to running hash
    // pub inputs: Vec<(u64, InputKind)>,
}

impl<F: PrimeField32> PartialMachineTrace<F> {
    pub fn get_trace_matrices(mut self) -> Vec<Option<RowMajorMatrix<F>>> {
        let mut range_counts = BTreeMap::new();
        let mut first_memory_reads = BTreeSet::new();

        self.memory.sort_by_key(|event| event.address);
        let mut memory_trace = vec![MemoryCols::default(); self.memory.len()];
        for (i, event) in self.memory.iter().enumerate() {
            memory_trace[i].addr = event.address;
            memory_trace[i].clk = event.clk;
            memory_trace[i].value = event.value;

            memory_trace[i].is_read = event.is_read;
            memory_trace[i].is_write = F::one() - event.is_read;

            let diff = if i > 0 {
                if memory_trace[i].addr == memory_trace[i - 1].addr {
                    memory_trace[i].addr_unchanged = F::one();
                    memory_trace[i].clk - memory_trace[i - 1].clk
                } else {
                    memory_trace[i].addr - memory_trace[i - 1].addr - F::one()
                }
            } else {
                F::zero()
            };

            if event.is_read == F::one() && (i == 0 || memory_trace[i].addr_unchanged == F::zero())
            {
                first_memory_reads.insert(event.address);
                memory_trace[i].is_first_read = F::one();
            }

            let diff_limb_lo = F::from_canonical_u32(diff.as_canonical_u32() % (1 << 8));
            let diff_limb_hi = F::from_canonical_u32((diff.as_canonical_u32() >> 8) % (1 << 8));

            memory_trace[i].diff_limb_lo = diff_limb_lo;
            memory_trace[i].diff_limb_hi = diff_limb_hi;

            range_counts
                .entry(diff_limb_lo)
                .and_modify(|count| *count += F::one())
                .or_insert(F::one());
            range_counts
                .entry(diff_limb_hi)
                .and_modify(|count| *count += F::one())
                .or_insert(F::one());
        }

        self.frame_buffer.sort_by_key(|event| event.address);
        let mut frame_buffer_trace = vec![FrameBufferCols::default(); self.frame_buffer.len()];
        for (i, event) in self.frame_buffer.iter().enumerate() {
            frame_buffer_trace[i].addr = event.address;
            frame_buffer_trace[i].clk = event.clk;
            frame_buffer_trace[i].value = event.value;

            frame_buffer_trace[i].is_read = event.is_read;
            frame_buffer_trace[i].is_write = F::one() - event.is_read;

            let diff = if i > 0 {
                if frame_buffer_trace[i].addr == frame_buffer_trace[i - 1].addr {
                    frame_buffer_trace[i].addr_unchanged = F::one();
                    frame_buffer_trace[i].clk - frame_buffer_trace[i - 1].clk
                } else {
                    frame_buffer_trace[i].addr - frame_buffer_trace[i - 1].addr - F::one()
                }
            } else {
                F::zero()
            };
            let diff_limb_lo = F::from_canonical_u32(diff.as_canonical_u32() % (1 << 8));
            let diff_limb_hi = F::from_canonical_u32((diff.as_canonical_u32() >> 8) % (1 << 8));

            frame_buffer_trace[i].diff_limb_lo = diff_limb_lo;
            frame_buffer_trace[i].diff_limb_hi = diff_limb_hi;

            range_counts
                .entry(diff_limb_lo)
                .and_modify(|count| *count += F::one())
                .or_insert(F::one());
            range_counts
                .entry(diff_limb_hi)
                .and_modify(|count| *count += F::one())
                .or_insert(F::one());
        }

        let range_trace = (0..(1 << 8))
            .map(|n| {
                let n = F::from_canonical_u32(n);
                RangeCols {
                    value: n,
                    mult: *range_counts.get(&n).unwrap_or(&F::zero()),
                }
            })
            .collect_vec();
        let memory_start_trace = (0..MEMORY_SIZE)
            .map(|n| MemoryStartCols {
                mult: F::from_bool(first_memory_reads.contains(&F::from_canonical_usize(n))),
            })
            .collect_vec();

        let cpu_matrix = self.cpu.to_trace_matrix(CpuCols::<F>::num_cols());
        let clear_matrix = self.clear.to_trace_matrix(ClearCols::<F>::num_cols());
        let draw_matrix = self.draw.to_trace_matrix(DrawCols::<F>::num_cols());
        let keypad_matrix = self.keypad.to_trace_matrix(KeypadCols::<F>::num_cols());
        let memory_matrix = memory_trace.to_trace_matrix(MemoryCols::<F>::num_cols());
        let frame_buffer_matrix =
            frame_buffer_trace.to_trace_matrix(FrameBufferCols::<F>::num_cols());
        let range_matrix = range_trace.to_trace_matrix(RangeCols::<F>::num_cols());
        let memory_start_matrix =
            memory_start_trace.to_trace_matrix(MemoryStartCols::<F>::num_cols());

        vec![
            cpu_matrix,
            clear_matrix,
            draw_matrix,
            keypad_matrix,
            memory_matrix,
            frame_buffer_matrix,
            range_matrix,
            memory_start_matrix,
        ]
    }
}

#[derive(Clone)]
pub struct IncrementalMachineTrace<F: PrimeField32> {
    pub cpu: IncrementalTrace<CpuCols<F>>,
    pub clear: IncrementalTrace<ClearCols<F>>,
    pub draw: IncrementalTrace<DrawCols<F>>,
    pub keypad: IncrementalTrace<KeypadCols<F>>,
    // range_trace: IncrementalTrace::default(),
    pub memory: Vec<MemoryEventLike<F>>,
    pub frame_buffer: Vec<MemoryEventLike<F>>,
    // TODO: Change to running hash
    // pub inputs: Vec<(u64, InputKind)>,
}

impl<F: PrimeField32> Default for IncrementalMachineTrace<F> {
    fn default() -> Self {
        let mut cpu: IncrementalTrace<CpuCols<F>> = IncrementalTrace::default();
        cpu.curr_row.program_counter = F::from_canonical_u16(PROGRAM_START_ADDRESS);

        Self {
            cpu,
            clear: IncrementalTrace::default(),
            draw: IncrementalTrace::default(),
            keypad: IncrementalTrace::default(),
            // range: IncrementalTrace::default(),
            memory: Vec::new(),
            frame_buffer: Vec::new(),
        }
    }
}

// TODO: Derive simple state from traces
pub struct StarkState<F: PrimeField32> {
    pub state: SimpleState,
    pub trace: IncrementalMachineTrace<F>,
}

impl<F: PrimeField32> Default for StarkState<F> {
    fn default() -> Self {
        Self {
            state: SimpleState::default(),
            trace: IncrementalMachineTrace::default(),
        }
    }
}

impl<F: PrimeField32> StarkState<F> {
    pub fn finalize_trace(&mut self) -> PartialMachineTrace<F> {
        // TODO: Remove clones
        let cpu = self.trace.cpu.trace.clone();
        let clear = self.trace.clear.trace.clone();
        let draw = self.trace.draw.trace.clone();
        let keypad = self.trace.keypad.trace.clone();

        PartialMachineTrace {
            cpu,
            clear,
            draw,
            keypad,
            memory: self.trace.memory.clone(),
            frame_buffer: self.trace.frame_buffer.clone(),
        }
    }
}

impl<F: PrimeField32> State for StarkState<F> {
    fn load_rom(&mut self, bytes: &[u8]) -> Result<(), Chip8Error> {
        self.state.load_rom(bytes)
    }

    fn clk(&self) -> Result<u64, Chip8Error> {
        self.state.clk()
    }

    fn clk_ptr(&self) -> Arc<RwLock<u64>> {
        self.state.clk_ptr()
    }

    fn sound_timer_ptr(&self) -> Arc<RwLock<Word>> {
        self.state.sound_timer_ptr()
    }

    fn frame_buffer_ptr(&self) -> Arc<RwLock<[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>> {
        self.state.frame_buffer_ptr()
    }

    fn program_counter(&self) -> Address {
        self.state.program_counter()
    }

    fn delay_timer(&self) -> Word {
        self.state.delay_timer()
    }

    fn sound_timer(&self) -> Result<Word, Chip8Error> {
        self.state.sound_timer()
    }

    fn memory(&mut self, addr: Address) -> Result<Word, Chip8Error> {
        let value = self.state.memory(addr)?;

        let clk = self.clk()?;
        let event = MemoryEventLike {
            clk: F::from_canonical_u64(clk),
            address: F::from_canonical_u16(addr),
            value: F::from_canonical_u8(value),
            is_read: F::from_bool(true),
        };
        self.trace.memory.push(event);

        Ok(value)
    }

    fn register(&self, index: Word) -> Word {
        self.state.register(index)
    }

    fn index_register(&self) -> Address {
        self.state.index_register()
    }

    fn key(&self, index: Word) -> bool {
        self.state.key(index)
    }

    fn frame_buffer(&mut self, y: usize, x: usize) -> Result<bool, Chip8Error> {
        let value = self.state.frame_buffer(y, x)?;

        let clk = self.clk()?;
        let addr = y * DISPLAY_WIDTH + x;
        let event = MemoryEventLike {
            clk: F::from_canonical_u64(clk),
            address: F::from_canonical_usize(addr),
            value: F::from_bool(value),
            is_read: F::from_bool(true),
        };
        self.trace.frame_buffer.push(event);

        Ok(value)
    }

    fn set_frame_buffer(&mut self, y: usize, x: usize, bit: bool) -> Result<(), Chip8Error> {
        let clk = self.clk()?;
        let addr = y * DISPLAY_WIDTH + x;
        let event = MemoryEventLike {
            clk: F::from_canonical_u64(clk),
            address: F::from_canonical_usize(addr),
            value: F::from_bool(bit),
            is_read: F::from_bool(false),
        };
        self.trace.frame_buffer.push(event);

        self.state.set_frame_buffer(y, x, bit)
    }

    fn set_program_counter(&mut self, pc: Address) {
        let next_row = &mut self.trace.cpu.next_row;
        next_row.program_counter = F::from_canonical_u16(pc);

        self.state.set_program_counter(pc)
    }

    fn set_delay_timer(&mut self, value: Word) {
        let curr_row = &mut self.trace.cpu.curr_row;
        curr_row.delay_timer = F::from_canonical_u8(value);

        self.state.set_delay_timer(value)
    }

    fn set_sound_timer(&mut self, value: Word) -> Result<(), Chip8Error> {
        let curr_row = &mut self.trace.cpu.curr_row;
        curr_row.sound_timer = F::from_canonical_u8(value);

        self.state.set_sound_timer(value)
    }

    fn set_index_register(&mut self, addr: Address) {
        let curr_row = &mut self.trace.cpu.curr_row;
        curr_row.index_register = F::from_canonical_u16(addr);

        self.state.set_index_register(addr)
    }

    fn set_register(&mut self, index: Word, value: Word) {
        let curr_row = &mut self.trace.cpu.curr_row;
        curr_row.registers[index as usize] = F::from_canonical_u8(value);

        self.state.set_register(index, value)
    }

    fn set_flag_register(&mut self, flag: bool) {
        let curr_row = &mut self.trace.cpu.curr_row;
        curr_row.registers[FLAG_REGISTER] = F::from_bool(flag);

        self.state.set_flag_register(flag)
    }

    fn set_memory(&mut self, addr: Address, value: Word) -> Result<(), Chip8Error> {
        let clk = self.clk()?;
        let event = MemoryEventLike {
            clk: F::from_canonical_u64(clk),
            address: F::from_canonical_u16(addr),
            value: F::from_canonical_u8(value),
            is_read: F::from_bool(false),
        };
        self.trace.memory.push(event);

        self.state.set_memory(addr, value)
    }

    fn set_key(&mut self, key: Key, kind: InputKind) {
        self.trace.cpu.curr_row.keypad[key as usize] = F::from_bool(kind == InputKind::Press);

        self.trace.keypad.curr_row.index = F::from_canonical_usize(key as usize);
        self.trace.keypad.curr_row.value = F::from_bool(kind == InputKind::Press);
        self.trace.keypad.add_curr_row_to_trace();

        self.state.set_key(key, kind)
    }

    fn clear_framebuffer(&mut self) -> Result<(), Chip8Error> {
        let clk = self.clk()?;
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let curr_row = &mut self.trace.clear.curr_row;
                curr_row.is_real = F::one();
                if y * DISPLAY_WIDTH + x == 0 {
                    curr_row.is_start = F::one();
                } else {
                    curr_row.is_start = F::zero();
                }
                curr_row.clk = F::from_canonical_u64(clk);
                curr_row.addr = F::from_canonical_usize(y * DISPLAY_WIDTH + x);

                self.trace.clear.add_curr_row_to_trace();

                let addr = y * DISPLAY_WIDTH + x;
                let event = MemoryEventLike {
                    clk: F::from_canonical_u64(clk),
                    address: F::from_canonical_usize(addr),
                    value: F::from_bool(false),
                    is_read: F::from_bool(false),
                };
                self.trace.frame_buffer.push(event);
            }
        }

        self.state.clear_framebuffer()
    }

    fn push_stack(&mut self, addr: Address) {
        let curr_row = &mut self.trace.cpu.curr_row;
        let next_row = &mut self.trace.cpu.next_row;
        curr_row.stack[self.state.stack_pointer as usize] =
            F::from_canonical_u16(self.state.program_counter);
        curr_row.stack_pointer += F::one();
        // TODO: Move this to a helper function
        for i in 0..STACK_DEPTH {
            if i == self.state.stack_pointer as usize + 1 {
                curr_row.stack_pointer_sel[i] = F::from_bool(true);
            } else {
                curr_row.stack_pointer_sel[i] = F::from_bool(false);
            }
        }
        next_row.program_counter = F::from_canonical_u16(addr);

        self.state.push_stack(addr)
    }

    fn pop_stack(&mut self) {
        let curr_row = &mut self.trace.cpu.curr_row;
        let next_row = &mut self.trace.cpu.next_row;
        curr_row.stack_pointer -= F::one();
        for i in 0..STACK_DEPTH {
            if i == self.state.stack_pointer as usize - 1 {
                curr_row.stack_pointer_sel[i] = F::from_bool(true);
            } else {
                curr_row.stack_pointer_sel[i] = F::from_bool(false);
            }
        }
        next_row.program_counter =
            F::from_canonical_u16(self.state.stack[self.state.stack_pointer as usize - 1]);

        self.state.pop_stack()
    }

    fn increment_program_counter(&mut self) {
        let next_row = &mut self.trace.cpu.next_row;
        next_row.program_counter += F::from_canonical_u16(OPCODE_SIZE);

        self.state.increment_program_counter()
    }

    fn increment_clk(&mut self) -> Result<(), Chip8Error> {
        let curr_row = &self.trace.cpu.curr_row;
        let next_row = &mut self.trace.cpu.next_row;
        next_row.clk = curr_row.clk + F::one();

        self.trace.cpu.add_curr_row_to_trace();

        self.state.increment_clk()
    }

    fn decrement_delay_timer(&mut self) {
        let curr_row = &mut self.trace.cpu.curr_row;
        curr_row.delay_timer -= F::one();

        self.state.decrement_delay_timer()
    }

    fn decrement_sound_timer(&mut self) -> Result<(), Chip8Error> {
        let curr_row = &mut self.trace.cpu.curr_row;
        curr_row.sound_timer -= F::one();

        self.state.decrement_sound_timer()
    }
}

impl<F: PrimeField32> IncrementalTrace<CpuCols<F>> {
    pub fn add_curr_row_to_trace(&mut self) {
        let vx = self
            .curr_row
            .x_sel
            .iter()
            .zip_eq(self.curr_row.registers.iter())
            .map(|(&sel, &register)| sel * register)
            .sum::<F>();
        let vy = self
            .curr_row
            .y_sel
            .iter()
            .zip_eq(self.curr_row.registers.iter())
            .map(|(&sel, &register)| sel * register)
            .sum::<F>();

        self.curr_row.vx = vx;
        self.curr_row.vy = vy;
        for i in 0..NUM_KEYS {
            self.curr_row.vx_sel[i] = F::from_bool(vx == F::from_canonical_usize(i));
        }
        self.curr_row.diff_vx_nn_inv = (vx - self.curr_row.nn).try_inverse().unwrap_or_default();
        self.curr_row.is_equal_vx_nn = F::from_bool(vx == self.curr_row.nn);
        self.curr_row.vx_bcd0 = F::from_canonical_u64((vx.as_canonical_u64() / 100) % 10);
        self.curr_row.vx_bcd1 = F::from_canonical_u64((vx.as_canonical_u64() / 10) % 10);
        self.curr_row.vx_bcd2 = F::from_canonical_u64(vx.as_canonical_u64() % 10);

        self.curr_row.diff_vx_vy_inv = (vx - vy).try_inverse().unwrap_or_default();
        self.curr_row.is_equal_vx_vy = F::from_bool(vx == vy);

        self.trace.push(self.curr_row.clone());
        // Copy state
        self.next_row.registers = self.curr_row.registers;
        self.next_row.index_register = self.curr_row.index_register;
        self.next_row.stack = self.curr_row.stack;
        self.next_row.stack_pointer = self.curr_row.stack_pointer;
        self.next_row.keypad = self.curr_row.keypad;
        self.next_row.stack_pointer_sel = self.curr_row.stack_pointer_sel;

        self.curr_row = self.next_row.clone();
        self.next_row = CpuCols::default();
    }
}

impl<F: PrimeField32> IncrementalTrace<KeypadCols<F>> {
    pub fn add_curr_row_to_trace(&mut self) {
        self.trace.push(self.curr_row.clone());
        self.curr_row = self.next_row.clone();

        self.next_row = KeypadCols::default();
    }
}

impl<F: PrimeField32> IncrementalTrace<DrawCols<F>> {
    pub fn add_curr_row_to_trace(&mut self) {
        self.trace.push(self.curr_row.clone());
        // Copy state
        self.next_row.is_real = self.curr_row.is_real;
        self.next_row.clk = self.curr_row.clk;
        self.next_row.register_x = self.curr_row.register_x;
        self.next_row.register_y = self.curr_row.register_y;
        self.next_row.index_register = self.curr_row.index_register;

        self.curr_row = self.next_row.clone();
        self.next_row = DrawCols::default();
    }
}

impl<F: PrimeField32> IncrementalTrace<ClearCols<F>> {
    pub fn add_curr_row_to_trace(&mut self) {
        self.trace.push(self.curr_row.clone());

        // Copy state
        self.next_row.is_real = self.curr_row.is_real;
        self.next_row.is_start = self.curr_row.is_start;
        self.next_row.clk = self.curr_row.clk;
        self.next_row.addr = self.curr_row.addr;

        self.curr_row = self.next_row.clone();
        self.next_row = ClearCols::default();
    }
}

pub trait ToTraceMatrix<F: PrimeField32> {
    fn to_trace_matrix(&self, num_cols: usize) -> Option<RowMajorMatrix<F>>;
}

impl<F: PrimeField32, Cols: Default + Clone> ToTraceMatrix<F> for Vec<Cols> {
    // TODO: Calculate num_cols from struct
    fn to_trace_matrix(&self, num_cols: usize) -> Option<RowMajorMatrix<F>> {
        if self.is_empty() {
            None
        } else {
            let mut trace = self.clone();
            let next_power_of_two = trace.len().next_power_of_two();
            trace.resize(next_power_of_two, Cols::default());

            let ptr = trace.as_ptr() as *const F;
            let len = trace.len() * num_cols;
            let values = unsafe { slice::from_raw_parts(ptr, len) };
            Some(RowMajorMatrix::new(values.to_vec(), num_cols))
        }
    }
}
