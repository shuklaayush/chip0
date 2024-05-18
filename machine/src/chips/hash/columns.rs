use core::mem::{size_of, transmute};

use p3_derive::AlignedBorrow;
use p3_util::indices_arr;

#[cfg(feature = "debug-trace")]
use p3_derive::Headers;

#[repr(C)]
#[derive(AlignedBorrow)]
#[cfg_attr(feature = "debug-trace", derive(Headers))]
pub struct HashCols<T> {
    is_real: T,
}

pub const NUM_HASH_COLS: usize = size_of::<HashCols<u8>>();
pub(crate) const HASH_COL_MAP: HashCols<usize> = make_col_map();

const fn make_col_map() -> HashCols<usize> {
    let indices_arr = indices_arr::<NUM_HASH_COLS>();
    unsafe { transmute::<[usize; NUM_HASH_COLS], HashCols<usize>>(indices_arr) }
}
