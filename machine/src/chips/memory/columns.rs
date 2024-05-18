use core::mem::{size_of, transmute};

use p3_derive::AlignedBorrow;
use p3_util::indices_arr;

#[cfg(feature = "debug-trace")]
use p3_derive::Headers;

#[repr(C)]
#[derive(AlignedBorrow)]
#[cfg_attr(feature = "debug-trace", derive(Headers))]
pub struct MemoryCols<T> {
    addr: T,
    clk: T,
    value: T,
    is_read: T,
    is_write: T,
    addr_unchanged: T,
    diff: T,
    is_first_read: T,
    is_last_write: T,
}

pub const NUM_MEMORY_COLS: usize = size_of::<MemoryCols<u8>>();
pub(crate) const MEMORY_COL_MAP: MemoryCols<usize> = make_col_map();

const fn make_col_map() -> MemoryCols<usize> {
    let indices_arr = indices_arr::<NUM_MEMORY_COLS>();
    unsafe { transmute::<[usize; NUM_MEMORY_COLS], MemoryCols<usize>>(indices_arr) }
}
