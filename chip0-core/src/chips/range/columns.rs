use core::mem::{size_of, transmute};

use p3_derive::AlignedBorrow;
use p3_util::indices_arr;

#[cfg(feature = "debug-trace")]
use p3_derive::Headers;

#[repr(C)]
#[derive(AlignedBorrow)]
#[cfg_attr(feature = "debug-trace", derive(Headers))]
pub struct RangeCols<T> {
    counter: T,
    mult: T,
}

pub const NUM_RANGE_COLS: usize = size_of::<RangeCols<u8>>();
pub(crate) const RANGE_COL_MAP: RangeCols<usize> = make_col_map();

const fn make_col_map() -> RangeCols<usize> {
    let indices_arr = indices_arr::<NUM_RANGE_COLS>();
    unsafe { transmute::<[usize; NUM_RANGE_COLS], RangeCols<usize>>(indices_arr) }
}
