use core::mem::{size_of, transmute};

use p3_derive::AlignedBorrow;
use p3_util::indices_arr;

#[cfg(feature = "debug-trace")]
use p3_derive::Headers;

#[repr(C)]
#[derive(AlignedBorrow)]
#[cfg_attr(feature = "debug-trace", derive(Headers))]
pub struct DrawCols<T> {
    is_real: T,
    clk: T,
    register_x: T,
    register_y: T,
    index_register: T,
    ys: T,
    y: T,
    pixels: T,
    xs: T,
    x: T,
    pixel: T,
    frame_buffer_y_x: T,
    flipped: T,
    register_flag: T,
}

pub const NUM_DRAW_COLS: usize = size_of::<DrawCols<u8>>();
pub(crate) const DRAW_COL_MAP: DrawCols<usize> = make_col_map();

const fn make_col_map() -> DrawCols<usize> {
    let indices_arr = indices_arr::<NUM_DRAW_COLS>();
    unsafe { transmute::<[usize; NUM_DRAW_COLS], DrawCols<usize>>(indices_arr) }
}
