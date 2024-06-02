use p3_derive::AirColumns;

pub const WORD_BITS: usize = 8;

#[repr(C)]
#[derive(AirColumns, Default, Clone)]
pub struct DrawCols<T> {
    pub is_real: T,
    pub is_first: T,
    pub is_last: T,

    // TODO: Replace with is_xs_0
    pub is_first_inner: T,

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

    pub pixels_bits: [T; WORD_BITS],
    pub sel_7_minus_xs: [T; WORD_BITS],
}
