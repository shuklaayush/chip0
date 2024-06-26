use p3_derive::Columnar;

#[repr(C)]
#[derive(Columnar, Default, Clone)]
pub struct KeypadCols<T> {
    pub is_real: T,
    pub clk: T,
    pub index: T,
    pub value: T,
    pub input_hash: T,
    pub output_hash: T,
}
