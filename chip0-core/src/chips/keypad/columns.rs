use core::mem::{size_of, transmute};

use p3_derive::AlignedBorrow;
use p3_util::indices_arr;

#[cfg(feature = "debug-trace")]
use p3_derive::Headers;

#[repr(C)]
#[derive(AlignedBorrow, Default, Copy, Clone)]
#[cfg_attr(feature = "debug-trace", derive(Headers))]
pub struct KeypadCols<T> {
    pub is_real: T,
    pub clk: T,
    pub index: T,
    pub value: T,
    pub input_hash: T,
    pub output_hash: T,
}

pub const NUM_KEYPAD_COLS: usize = size_of::<KeypadCols<u8>>();
pub(crate) const KEYPAD_COL_MAP: KeypadCols<usize> = make_col_map();

const fn make_col_map() -> KeypadCols<usize> {
    let indices_arr = indices_arr::<NUM_KEYPAD_COLS>();
    unsafe { transmute::<[usize; NUM_KEYPAD_COLS], KeypadCols<usize>>(indices_arr) }
}
