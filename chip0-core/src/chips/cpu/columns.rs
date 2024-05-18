use core::mem::{size_of, transmute};

use chip8_core::constants::{NUM_KEYS, NUM_REGISTERS, STACK_DEPTH};
use p3_derive::AlignedBorrow;
use p3_util::indices_arr;

#[cfg(feature = "debug-trace")]
use p3_derive::Headers;

#[repr(C)]
#[derive(AlignedBorrow)]
#[cfg_attr(feature = "debug-trace", derive(Headers))]
pub struct CpuCols<T> {
    is_real: T,
    is_first: T,
    is_last: T,

    program_counter: T,
    registers: [T; NUM_REGISTERS],
    index_register: T,
    stack: [T; STACK_DEPTH],
    stack_pointer: T,
    delay_timer: T,
    sound_timer: T,
    keypad: [T; NUM_KEYS],

    clk: T,
    instruction: T,

    is_clear_display: T,
    is_return: T,
    is_jump: T,
    is_call: T,
    is_skip_equal: T,
    is_skip_not_equal: T,
    is_skip_equal_xy: T,
    is_load: T,
    is_add: T,
    is_move: T,
    is_or: T,
    is_xor: T,
    is_add_xy: T,
    is_sub_xy: T,
    is_shift_right: T,
    is_sub_yx: T,
    is_shift_left: T,
    is_skip_not_equal_xy: T,
    is_load_i: T,
    is_jump_v0: T,
    is_random: T,
    is_draw: T,
    is_skip_key_pressed: T,
    is_skip_key_not_pressed: T,
    is_load_delay: T,
    is_wait_key_press: T,
    is_set_delay: T,
    is_set_sound: T,
    is_add_i: T,
    is_load_font: T,
    is_store_bcd: T,
    is_store_registers: T,
    is_load_memory: T,

    x: T,
    y: T,
    n: T,
    nn: T,
    nnn: T,

    x_bits: [T; NUM_REGISTERS],
    y_bits: [T; NUM_REGISTERS],

    // x <= i
    x_lte: [T; NUM_REGISTERS],
}

pub const NUM_CPU_COLS: usize = size_of::<CpuCols<u8>>();
pub(crate) const CPU_COL_MAP: CpuCols<usize> = make_col_map();

const fn make_col_map() -> CpuCols<usize> {
    let indices_arr = indices_arr::<NUM_CPU_COLS>();
    unsafe { transmute::<[usize; NUM_CPU_COLS], CpuCols<usize>>(indices_arr) }
}
