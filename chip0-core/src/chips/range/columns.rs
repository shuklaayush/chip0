use p3_derive::Columnar;

#[repr(C)]
#[derive(Columnar, Default, Clone)]
pub struct RangeCols<T> {
    pub value: T,
    pub mult: T,
}
