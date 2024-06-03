use p3_derive::Columnar;

// TODO: Use preprocessed?
#[repr(C)]
#[derive(Columnar, Default, Clone)]
pub struct ClearCols<T> {
    pub is_real: T,
    pub is_start: T,
    pub clk: T,
    pub addr: T,
}
