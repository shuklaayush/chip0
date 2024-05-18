use core::mem::{size_of, transmute};

use p3_derive::AlignedBorrow;
use p3_util::indices_arr;

#[cfg(feature = "debug-trace")]
use p3_derive::Headers;

#[repr(C)]
#[derive(AlignedBorrow, Default, Copy, Clone)]
#[cfg_attr(feature = "debug-trace", derive(Headers))]
pub struct DrawCols<T> {
    pub is_real: T,
    pub clk: T,
    pub register_x: T,
    pub register_y: T,
    pub index_register: T,
    pub ys: T,
    pub y: T,
    pub pixels: T,
    pub xs: T,
    pub x: T,
    pub pixel: T,
    pub frame_buffer_y_x: T,
    pub flipped: T,
    pub register_flag: T,
}

pub const NUM_DRAW_COLS: usize = size_of::<DrawCols<u8>>();
pub(crate) const DRAW_COL_MAP: DrawCols<usize> = make_col_map();

const fn make_col_map() -> DrawCols<usize> {
    let indices_arr = indices_arr::<NUM_DRAW_COLS>();
    unsafe { transmute::<[usize; NUM_DRAW_COLS], DrawCols<usize>>(indices_arr) }
}
