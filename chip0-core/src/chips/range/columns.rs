use p3_derive::AirColumns;

#[repr(C)]
#[derive(AirColumns, Default, Clone)]
pub struct RangeCols<T> {
    pub value: T,
    pub mult: T,
}
