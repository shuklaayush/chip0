use chip8_core::constants::{NUM_KEYS, NUM_REGISTERS, STACK_DEPTH};
use p3_derive::Columnar;

#[repr(C)]
#[derive(Columnar, Clone, Default)]
pub struct CpuCols<T> {
    pub is_real: T,

    pub clk: T,
    pub opcode_hi: T,
    pub opcode_lo: T,

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

    pub vx: T,
    pub vy: T,

    pub vx_bcd0: T,
    pub vx_bcd1: T,
    pub vx_bcd2: T,

    pub vx_sel: [T; NUM_KEYS],

    // x <= i
    pub lte_x_sel: [T; NUM_REGISTERS],

    pub diff_vx_nn_inv: T,
    pub is_equal_vx_nn: T,

    pub diff_vx_vy_inv: T,
    pub is_equal_vx_vy: T,

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
    // pub add_vi_vx: T,
    pub is_first: T,
    pub is_final: T,
}
