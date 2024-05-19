use core::mem::{size_of, transmute};

use p3_derive::AlignedBorrow;
use p3_util::indices_arr;

#[cfg(feature = "debug-trace")]
use p3_derive::Headers;

#[repr(C)]
#[derive(AlignedBorrow, Default, Copy, Clone)]
#[cfg_attr(feature = "debug-trace", derive(Headers))]
pub struct MemoryStartPreprocessedCols<T> {
    pub addr: T,
    pub value: T,
}

#[repr(C)]
#[derive(AlignedBorrow, Default, Copy, Clone)]
#[cfg_attr(feature = "debug-trace", derive(Headers))]
pub struct MemoryStartCols<T> {
    pub mult: T,
}

pub const NUM_MEMORY_START_PREPROCESSED_COLS: usize = size_of::<MemoryStartPreprocessedCols<u8>>();
pub(crate) const MEMORY_START_PREPROCESSED_COL_MAP: MemoryStartPreprocessedCols<usize> =
    make_preprocessed_col_map();

const fn make_preprocessed_col_map() -> MemoryStartPreprocessedCols<usize> {
    let indices_arr = indices_arr::<NUM_MEMORY_START_PREPROCESSED_COLS>();
    unsafe {
        transmute::<[usize; NUM_MEMORY_START_PREPROCESSED_COLS], MemoryStartPreprocessedCols<usize>>(
            indices_arr,
        )
    }
}
pub const NUM_MEMORY_START_COLS: usize = size_of::<MemoryStartCols<u8>>();
pub(crate) const MEMORY_START_COL_MAP: MemoryStartCols<usize> = make_col_map();

const fn make_col_map() -> MemoryStartCols<usize> {
    let indices_arr = indices_arr::<NUM_MEMORY_START_COLS>();
    unsafe { transmute::<[usize; NUM_MEMORY_START_COLS], MemoryStartCols<usize>>(indices_arr) }
}
