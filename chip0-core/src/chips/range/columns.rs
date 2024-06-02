use p3_derive::Columns;

#[repr(C)]
#[derive(Columns, Default, Clone)]
pub struct RangeCols<T> {
    pub value: T,
    pub mult: T,
}
