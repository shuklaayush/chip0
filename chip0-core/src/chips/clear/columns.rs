use p3_derive::Columnar;

pub const WORD_BITS: usize = 8;

#[repr(C)]
#[derive(Columnar, Default, Clone)]
pub struct ClearCols<T> {
    pub is_real: T,
    pub clk: T,
    pub x: T,
    pub y: T,
}
