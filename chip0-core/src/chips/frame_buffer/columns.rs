use core::mem::{size_of, transmute};

use p3_derive::AlignedBorrow;
use p3_util::indices_arr;

#[cfg(feature = "debug-trace")]
use p3_derive::Headers;

#[repr(C)]
#[derive(AlignedBorrow, Default, Copy, Clone)]
#[cfg_attr(feature = "debug-trace", derive(Headers))]
pub struct FrameBufferCols<T> {
    pub addr: T,
    pub clk: T,
    pub value: T,
    pub is_read: T,
    pub is_write: T,
    pub addr_unchanged: T,
    pub diff_limb_lo: T,
    pub diff_limb_hi: T,
    pub is_first_read: T,
    pub is_last_write: T,
}

pub const NUM_FRAME_BUFFER_COLS: usize = size_of::<FrameBufferCols<u8>>();
pub(crate) const FRAME_BUFFER_COL_MAP: FrameBufferCols<usize> = make_col_map();

const fn make_col_map() -> FrameBufferCols<usize> {
    let indices_arr = indices_arr::<NUM_FRAME_BUFFER_COLS>();
    unsafe { transmute::<[usize; NUM_FRAME_BUFFER_COLS], FrameBufferCols<usize>>(indices_arr) }
}