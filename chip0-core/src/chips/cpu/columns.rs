use core::mem::{size_of, transmute};

use chip8_core::constants::{NUM_KEYS, NUM_REGISTERS, STACK_DEPTH};
use p3_derive::AlignedBorrow;
use p3_util::indices_arr;

#[cfg(feature = "debug-trace")]
use p3_derive::Headers;

#[repr(C)]
#[derive(AlignedBorrow, Default, Copy, Clone)]
#[cfg_attr(feature = "debug-trace", derive(Headers))]
pub struct CpuCols<T> {
    pub is_real: T,

    pub clk: T,
    pub opcode: T,

    pub is_clear_display: T,
    pub is_return: T,
    pub is_jump: T,
    pub is_call: T,
    pub is_skip_equal: T,
    pub is_skip_not_equal: T,
    pub is_skip_equal_xy: T,
    pub is_load: T,
    pub is_add: T,
    pub is_move: T,
    pub is_or: T,
    pub is_and: T,
    pub is_xor: T,
    pub is_add_xy: T,
    pub is_sub_xy: T,
    pub is_shift_right: T,
    pub is_sub_yx: T,
    pub is_shift_left: T,
    pub is_skip_not_equal_xy: T,
    pub is_load_i: T,
    pub is_jump_v0: T,
    pub is_random: T,
    pub is_draw: T,
    pub is_skip_key_pressed: T,
    pub is_skip_key_not_pressed: T,
    pub is_load_delay: T,
    pub is_wait_key_press: T,
    pub is_set_delay: T,
    pub is_set_sound: T,
    pub is_add_i: T,
    pub is_load_font: T,
    pub is_store_bcd: T,
    pub is_store_registers: T,
    pub is_load_memory: T,

    pub program_counter: T,
    pub registers: [T; NUM_REGISTERS],
    pub index_register: T,
    pub stack: [T; STACK_DEPTH],
    pub stack_pointer: T,
    pub delay_timer: T,
    pub sound_timer: T,
    pub keypad: [T; NUM_KEYS],

    pub stack_pointer_sel: [T; STACK_DEPTH],

    pub x: T,
    pub y: T,
    pub n: T,
    pub nn: T,
    pub nnn: T,

    pub x_sel: [T; NUM_REGISTERS],
    pub y_sel: [T; NUM_REGISTERS],

    // x <= i
    pub lte_x_sel: [T; NUM_REGISTERS],

    // pub is_equal_vx_nn: T,
    // pub is_equal_vx_vy: T,
    // pub or_vx_vy: T,
    // pub and_vx_vy: T,
    // pub xor_vx_vy: T,
    // pub add_vx_vy: T,
    // pub add_vx_vy_carry: T,
    // pub sub_vx_vy: T,
    // pub sub_vx_vy_borrow: T,
    // pub shr_vx: T,
    // pub shr_vx_flag: T,
    // pub sub_vy_vx: T,
    // pub sub_vy_vx_borrow: T,
    // pub shl_vx: T,
    // pub shl_vx_flag: T,
    // pub is_key_pressed_vx: T,
    // pub add_vi_vx: T,
    pub is_first: T,
    pub is_final: T,
}

pub const NUM_CPU_COLS: usize = size_of::<CpuCols<u8>>();
pub(crate) const CPU_COL_MAP: CpuCols<usize> = make_col_map();

const fn make_col_map() -> CpuCols<usize> {
    let indices_arr = indices_arr::<NUM_CPU_COLS>();
    unsafe { transmute::<[usize; NUM_CPU_COLS], CpuCols<usize>>(indices_arr) }
}
